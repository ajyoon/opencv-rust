use opencv::{
    core::{self, CV_32S, CV_32SC3, CV_64F, CV_64FC1, CV_8U, CV_8UC2, MAKETYPE, Moments},
    core::{Point2f, RotatedRect, Size2f},
    Result,
};

#[test]
fn make_type() {
    assert_eq!(MAKETYPE(CV_8U, 2), CV_8UC2);
    assert_eq!(MAKETYPE(CV_32S, 3), CV_32SC3);
    assert_eq!(MAKETYPE(CV_64F, 1), CV_64FC1);
}

#[test]
fn moments() -> Result<()> {
    let moments = Moments::default()?;
    assert_eq!(0., moments.m00);
    assert_eq!(0., moments.m12);
    assert_eq!(0., moments.mu30);
    Ok(())
}

#[test]
fn cpu_features_line() -> Result<()> {
    let cpu_feats = core::get_cpu_features_line()?;
    assert!(cpu_feats.is_ascii());
    Ok(())
}

#[test]
fn rotated_rect() -> Result<()> {
    let rect = RotatedRect::new(Point2f::new(100., 100.), Size2f::new(100., 100.), 90.)?;
    let mut pts = [Point2f::default(); 4];
    rect.points(&mut pts)?;
    assert_eq!(Point2f::new(50., 50.), pts[0]);
    assert_eq!(Point2f::new(150., 50.), pts[1]);
    assert_eq!(Point2f::new(150., 150.), pts[2]);
    assert_eq!(Point2f::new(50., 150.), pts[3]);
    Ok(())
}
