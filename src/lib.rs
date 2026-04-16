/*!
A trait-based implementation of the xpans Audio Source Definition for use
across xpans-compatible crates.
*/
#![no_std]
/**
Access the position property of an audio source

All values should be finite and non-NaN.
*/
pub trait Position<T> {
    /**
    Left/Right position coordinate

    A negative value indicates a leftward position,
    a positive value indicates a rightward position.
    */
    fn pos_x(&self) -> T;
    /**
    Backwards/Forwards position coordinate

    A negative value indicates a backward position,
    a positive value indicates a forward position.
    */
    fn pos_y(&self) -> T;
    /**
    Down/Up position coordinate

    A negative value indicates a downward position,
    a positive value indicates an upward position.
    */
    fn pos_z(&self) -> T;
}

/**
Access the extent property of an audio source

All values should be positive, finite, and non-NaN.
*/
pub trait Extent<T> {
    /**
    Left/Right extent coordinate

    This value describes the scale of the audio source positively and
    negatively along the X axis.
    */
    fn ext_x(&self) -> T;
    /**
    Backwards/Forwards extent coordinate

    This value describes the scale of the audio source positively and
    negatively along the Y axis.
    */
    fn ext_y(&self) -> T;
    /**
    Down/Up extent coordinate

    This value describes the scale of the audio source positively and
    negatively along the Z axis.
    */
    fn ext_z(&self) -> T;
}
