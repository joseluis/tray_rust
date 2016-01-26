//! This module provides the Torrance Sparrow microfacet BRDF
//! TODO: Wikipedia link?

use std::f32;
use enum_set::EnumSet;

use linalg::{self, Vector};
use film::Colorf;
use bxdf::{self, BxDF, BxDFType};
use bxdf::fresnel::Fresnel;
use bxdf::microfacet::{MicrofacetDistribution};

/// Struct providing the Torrance Sparrow BRDF
pub struct TorranceSparrow {
    reflectance: Colorf,
    fresnel: Box<Fresnel + Send + Sync>,
    /// Microfacet distribution describing the structure of the microfacets of
    /// the material
    microfacet: Box<MicrofacetDistribution + Send + Sync>,
}

impl TorranceSparrow {
    /// Create a new Torrance Sparrow microfacet BRDF
    pub fn new(c: &Colorf, fresnel: Box<Fresnel + Send + Sync>,
               microfacet: Box<MicrofacetDistribution + Send + Sync>) -> TorranceSparrow {
        TorranceSparrow { reflectance: *c, fresnel: fresnel, microfacet: microfacet }
    }
}

impl BxDF for TorranceSparrow {
    fn bxdf_type(&self) -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Glossy);
        e.insert(BxDFType::Reflection);
        e
    }
    fn eval(&self, w_o: &Vector, w_i: &Vector) -> Colorf {
        let cos_to = f32::abs(bxdf::cos_theta(w_o));
        let cos_ti = f32::abs(bxdf::cos_theta(w_i));
        if cos_to == 0.0 || cos_ti == 0.0 {
            return Colorf::new(0.0, 0.0, 0.0)
        }
        let mut w_h = *w_i + *w_o;
        if w_h.x == 0.0 && w_h.y == 0.0 && w_h.z == 0.0 {
            return Colorf::new(0.0, 0.0, 0.0)
        }
        w_h = w_h.normalized();
        let d = self.microfacet.normal_distribution(&w_h);
        let f = self.fresnel.fresnel(linalg::dot(w_i, &w_h));
        let g = self.microfacet.shadowing_masking(w_i, w_o, &w_h);
        (self.reflectance * f * d * g / (4.0 * cos_ti * cos_to))
    }
    fn sample(&self, w_o: &Vector, samples: &(f32, f32)) -> (Colorf, Vector, f32) {
        let w_h = self.microfacet.sample(w_o, samples);
        let w_i = linalg::reflect(w_o, &w_h);
        if !bxdf::same_hemisphere(w_o, &w_i) {
            (Colorf::black(), Vector::broadcast(0.0), 0.0)
        } else {
            let pdf = self.microfacet.pdf(&w_h);
            /*
            println!("****\nMicrofacet pdf = {}, will divide by {}, w_h = {:?}\nw_o = {:?}\nw_i = {:?}\n******",
                     pdf, 4.0 * linalg::dot(w_o, &w_h), w_h, w_o, w_i);
                     */
            (self.eval(w_o, &w_i), w_i, self.microfacet.pdf(&w_h) / (4.0 * f32::abs(linalg::dot(w_o, &w_h))))
        }
    }
    fn pdf(&self, w_o: &Vector, w_i: &Vector) -> f32 {
        if !bxdf::same_hemisphere(w_o, w_i) {
            0.0
        } else {
            let w_h = *w_o + *w_i;
            if w_h.x == 0.0 && w_h.y == 0.0 && w_h.z == 0.0 {
                0.0
            } else {
                self.microfacet.pdf(&w_h.normalized()) / (4.0 * f32::abs(linalg::dot(w_o, &w_h)))
            }
        }
    }
}

