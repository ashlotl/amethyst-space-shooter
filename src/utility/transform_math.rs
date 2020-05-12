use amethyst::{
	core::{
		math::base::{
			ArrayStorage,
			Matrix,
		},
		math::U1,
		math::U3,
	},
};



pub fn dist_sqrd(one:Matrix<f32,U3,U1,ArrayStorage<f32,U3,U1>>, two:Matrix<f32,U3,U1,ArrayStorage<f32,U3,U1>>) -> f32 {
	return (one.x-two.x).powi(2)+(one.y-two.y).powi(2)+(one.z-two.z).powi(2)
}
