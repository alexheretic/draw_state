// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Fixed-function hardware state.
//!
//! Configures the primitive assembly (PA), rasterizer, and output merger (OM) blocks.

use std::default::Default;
use std::fmt;

use target;

/// The front face winding order of a set of vertices.
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
pub enum FrontFace {
    /// Clockwise winding order.
    Clockwise,
    /// Counter-clockwise winding order.
    CounterClockwise,
}

/// Width of a line.
pub type LineWidth = f32;
#[allow(missing_docs)]
pub type OffsetFactor = f32;
#[allow(missing_docs)]
pub type OffsetUnits = u32;

/// How to offset vertices in screen space, if at all.
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct Offset(pub OffsetFactor, pub OffsetUnits);

/// Which face, if any, to cull.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/// How to rasterize a primitive.
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub enum RasterMethod {
    /// Rasterize as a point.
    Point,
    /// Rasterize as a line with the given width.
    Line(LineWidth),
    /// Rasterize as a face with a given cull face mode.
    Fill(CullFace),
}

/// Primitive rasterization state. Note that GL allows different raster
/// method to be used for front and back, while this abstraction does not.
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct Primitive {
    /// Which vertex winding is considered to be the front face for culling.
    pub front_face: FrontFace,
    /// How to rasterize this primitive.
    pub method: RasterMethod,
    /// Any polygon offset to apply.
    pub offset: Option<Offset>,
}

impl Primitive {
    /// Get the cull face, if any, for this primitive state.
    pub fn get_cull_face(&self) -> CullFace {
        match self.method {
            RasterMethod::Fill(mode) => mode,
            _ => CullFace::Nothing,
        }
    }
}

impl Default for Primitive {
    fn default() -> Primitive {
        Primitive {
            front_face: FrontFace::CounterClockwise,
            method: RasterMethod::Fill(CullFace::Nothing),
            offset: None,
        }
    }
}

/// Multi-sampling rasterization mode
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct MultiSample;
    //sample_mask: u16,
    //alpha_to_coverage: bool,

/// A pixel-wise comparison function.
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum Comparison {
    /// `false`
    Never,
    /// `x < y`
    Less,
    /// `x <= y`
    LessEqual,
    /// `x == y`
    Equal,
    /// `x >= y`
    GreaterEqual,
    /// `x > y`
    Greater,
    /// `x != y`
    NotEqual,
    /// `true`
    Always,
}

/// Stencil mask operation.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum StencilOp {
    /// Keep the current value in the stencil buffer (no change).
    Keep,
    /// Set the value in the stencil buffer to zero.
    Zero,
    /// Set the stencil buffer value to `value` from `StencilSide`
    Replace,
    /// Increment the stencil buffer value, clamping to its maximum value.
    IncrementClamp,
    /// Increment the stencil buffer value, wrapping around to 0 on overflow.
    IncrementWrap,
    /// Decrement the stencil buffer value, clamping to its minimum value.
    DecrementClamp,
    /// Decrement the stencil buffer value, wrapping around to the maximum value on overflow.
    DecrementWrap,
    /// Bitwise invert the current value in the stencil buffer.
    Invert,
}

/// Complete stencil state for a given side of a face.
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct StencilSide {
    /// Comparison function to use to determine if the stencil test passes.
    pub fun: Comparison,
    /// Reference value to compare the value in the stencil buffer with.
    pub value: target::Stencil,
    /// A mask that is ANDd with both the stencil buffer value and the reference value when they
    /// are read before doing the stencil test.
    pub mask_read: target::Stencil,
    /// A mask that is ANDd with the stencil value before writing to the stencil buffer.
    pub mask_write: target::Stencil,
    /// What operation to do if the stencil test fails.
    pub op_fail: StencilOp,
    /// What operation to do if the stenil test passes but the depth test fails.
    pub op_depth_fail: StencilOp,
    /// What operation to do if both the depth and stencil test pass.
    pub op_pass: StencilOp,
}

impl Default for StencilSide {
    fn default() -> StencilSide {
        StencilSide {
            fun: Comparison::Always,
            value: 0,
            mask_read: target::Stencil::max_value(),
            mask_write: target::Stencil::max_value(),
            op_fail: StencilOp::Keep,
            op_depth_fail: StencilOp::Keep,
            op_pass: StencilOp::Keep,
        }
    }
}

/// Complete stencil state, specifying how to handle the front and back side of a face.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct Stencil {
    pub front: StencilSide,
    pub back: StencilSide,
}

/// Depth test state.
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct Depth {
    /// Comparison function to use.
    pub fun: Comparison,
    /// Specify whether to write to the depth buffer or not.
    pub write: bool,
}

impl Default for Depth {
    fn default() -> Depth {
        Depth {
            fun: Comparison::Always,
            write: false,
        }
    }
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum Equation {
    /// Adds source and destination.
    /// Source and destination are multiplied by blending parameters before addition.
    Add,
    /// Subtracts destination from source.
    /// Source and destination are multiplied by blending parameters before subtraction.
    Sub,
    /// Subtracts source from destination.
    /// Source and destination are multiplied by blending parameters before subtraction.
    RevSub,
    /// Component-wise minimum value of source and destination.
    /// Blending parameters are ignored.
    Min,
    /// Component-wise maximum value of source and destination.
    /// Blending parameters are ignored.
    Max,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestColor,
    DestAlpha,
    ConstColor,
    ConstAlpha,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord)]
pub enum Factor {
    Zero,
    One,
    SourceAlphaSaturated,
    ZeroPlus(BlendValue),
    OneMinus(BlendValue),
}

#[allow(missing_docs)]
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Debug)]
pub struct BlendChannel {
    pub equation: Equation,
    pub source: Factor,
    pub destination: Factor,
}

impl Default for BlendChannel {
    fn default() -> BlendChannel {
        BlendChannel {
            equation: Equation::Add,
            source: Factor::One,
            destination: Factor::One,
        }
    }
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Blend {
    pub color: BlendChannel,
    pub alpha: BlendChannel,
    pub value: target::ColorValue,
}

impl Default for Blend {
    fn default() -> Blend {
        Blend {
            color: Default::default(),
            alpha: Default::default(),
            value: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl fmt::Debug for Blend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blend {{ color: {:?}, alpha: {:?}, value: {:?} }}",
               self.color, self.alpha, &self.value[..])
    }
}

bitflags!(
    #[allow(missing_docs)]
    flags ColorMask: u32 {  //u8 is preferred, but doesn't seem to work well
        const RED     = 0x1,
        const GREEN   = 0x2,
        const BLUE    = 0x4,
        const ALPHA   = 0x8,
        const MASK_ALL = 0xF,
        const MASK_NONE = 0x0
    }
);
