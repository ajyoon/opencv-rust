use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{core, types};

pub struct PtrOfAKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAKAZE {
    #[inline(always)] pub fn as_raw_PtrOfAKAZE(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAKAZE {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::AKAZE>*"] {
            delete me;
        })
    }
}
impl crate::features2d::AKAZE for PtrOfAKAZE {
    #[inline(always)] fn as_raw_AKAZE(&self) -> *mut c_void {
        let mePtrOfAKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAKAZE as "cv::Ptr<cv::AKAZE>*"] -> *mut c_void as "void*" {
            return mePtrOfAKAZE->get();
        })
    }
}

impl core::Algorithm for PtrOfAKAZE {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfAKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAKAZE as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfAKAZE->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfAKAZE {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfAKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAKAZE as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfAKAZE->get();
        })
    }
}

pub struct PtrOfANN_MLP {
    pub(crate) ptr: *mut c_void
}

impl PtrOfANN_MLP {
    #[inline(always)] pub fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfANN_MLP {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::ANN_MLP>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfANN_MLP {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfANN_MLP = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfANN_MLP as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfANN_MLP->get();
        })
    }
}

impl crate::ml::ANN_MLP for PtrOfANN_MLP {
    #[inline(always)] fn as_raw_ANN_MLP(&self) -> *mut c_void {
        let mePtrOfANN_MLP = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfANN_MLP as "cv::Ptr<cv::ml::ANN_MLP>*"] -> *mut c_void as "void*" {
            return mePtrOfANN_MLP->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfANN_MLP {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfANN_MLP = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfANN_MLP as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfANN_MLP->get();
        })
    }
}

pub struct PtrOfAbsLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAbsLayer {
    #[inline(always)] pub fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAbsLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::AbsLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfActivationLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfActivationLayer {
    #[inline(always)] pub fn as_raw_PtrOfActivationLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfActivationLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ActivationLayer>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfActivationLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfActivationLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfActivationLayer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfActivationLayer->get();
        })
    }
}

impl crate::dnn::ActivationLayer for PtrOfActivationLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void {
        let mePtrOfActivationLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfActivationLayer as "cv::Ptr<cv::dnn::ActivationLayer>*"] -> *mut c_void as "void*" {
            return mePtrOfActivationLayer->get();
        })
    }
}

impl crate::dnn::Layer for PtrOfActivationLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
        let mePtrOfActivationLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfActivationLayer as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
            return mePtrOfActivationLayer->get();
        })
    }
}

pub struct PtrOfAffineTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAffineTransformer {
    #[inline(always)] pub fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAffineTransformer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::AffineTransformer>*"] {
            delete me;
        })
    }
}
impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
    #[inline(always)] fn as_raw_AffineTransformer(&self) -> *mut c_void {
        let mePtrOfAffineTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAffineTransformer as "cv::Ptr<cv::AffineTransformer>*"] -> *mut c_void as "void*" {
            return mePtrOfAffineTransformer->get();
        })
    }
}

impl core::Algorithm for PtrOfAffineTransformer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfAffineTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAffineTransformer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfAffineTransformer->get();
        })
    }
}

impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
    #[inline(always)] fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        let mePtrOfAffineTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAffineTransformer as "cv::Ptr<cv::ShapeTransformer>*"] -> *mut c_void as "void*" {
            return mePtrOfAffineTransformer->get();
        })
    }
}

pub struct PtrOfAgastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAgastFeatureDetector {
    #[inline(always)] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAgastFeatureDetector {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::AgastFeatureDetector>*"] {
            delete me;
        })
    }
}
impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
    #[inline(always)] fn as_raw_AgastFeatureDetector(&self) -> *mut c_void {
        let mePtrOfAgastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAgastFeatureDetector as "cv::Ptr<cv::AgastFeatureDetector>*"] -> *mut c_void as "void*" {
            return mePtrOfAgastFeatureDetector->get();
        })
    }
}

impl core::Algorithm for PtrOfAgastFeatureDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfAgastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAgastFeatureDetector as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfAgastFeatureDetector->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfAgastFeatureDetector {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfAgastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAgastFeatureDetector as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfAgastFeatureDetector->get();
        })
    }
}

pub struct PtrOfAlignMTB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAlignMTB {
    #[inline(always)] pub fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAlignMTB {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::AlignMTB>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfAlignMTB {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfAlignMTB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAlignMTB as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfAlignMTB->get();
        })
    }
}

impl crate::photo::AlignExposures for PtrOfAlignMTB {
    #[inline(always)] fn as_raw_AlignExposures(&self) -> *mut c_void {
        let mePtrOfAlignMTB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAlignMTB as "cv::Ptr<cv::AlignExposures>*"] -> *mut c_void as "void*" {
            return mePtrOfAlignMTB->get();
        })
    }
}

impl crate::photo::AlignMTB for PtrOfAlignMTB {
    #[inline(always)] fn as_raw_AlignMTB(&self) -> *mut c_void {
        let mePtrOfAlignMTB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfAlignMTB as "cv::Ptr<cv::AlignMTB>*"] -> *mut c_void as "void*" {
            return mePtrOfAlignMTB->get();
        })
    }
}

pub struct PtrOfAverageHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfAverageHash {
    #[inline(always)] pub fn as_raw_PtrOfAverageHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfAverageHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::AverageHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBFMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBFMatcher {
    #[inline(always)] pub fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBFMatcher {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::BFMatcher>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBNLLLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBNLLLayer {
    #[inline(always)] pub fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBNLLLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::BNLLLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBRISK {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBRISK {
    #[inline(always)] pub fn as_raw_PtrOfBRISK(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBRISK {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::BRISK>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBackendNode {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendNode {
    #[inline(always)] pub fn as_raw_PtrOfBackendNode(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBackendNode {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::BackendNode>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackendWrapper {
    #[inline(always)] pub fn as_raw_PtrOfBackendWrapper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBackendWrapper {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::BackendWrapper>*"] {
            delete me;
        })
    }
}
impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
    #[inline(always)] fn as_raw_BackendWrapper(&self) -> *mut c_void {
        let mePtrOfBackendWrapper = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackendWrapper as "cv::Ptr<cv::dnn::BackendWrapper>*"] -> *mut c_void as "void*" {
            return mePtrOfBackendWrapper->get();
        })
    }
}

pub struct PtrOfBackgroundSubtractorKNN {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorKNN {
    #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBackgroundSubtractorKNN {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::BackgroundSubtractorKNN>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfBackgroundSubtractorKNN {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorKNN = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorKNN as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorKNN->get();
        })
    }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
    #[inline(always)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorKNN = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorKNN as "cv::Ptr<cv::BackgroundSubtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorKNN->get();
        })
    }
}

impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
    #[inline(always)] fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorKNN = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorKNN as "cv::Ptr<cv::BackgroundSubtractorKNN>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorKNN->get();
        })
    }
}

pub struct PtrOfBackgroundSubtractorMOG2 {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBackgroundSubtractorMOG2 {
    #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBackgroundSubtractorMOG2 {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::BackgroundSubtractorMOG2>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfBackgroundSubtractorMOG2 {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorMOG2 = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorMOG2 as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorMOG2->get();
        })
    }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
    #[inline(always)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorMOG2 = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorMOG2 as "cv::Ptr<cv::BackgroundSubtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorMOG2->get();
        })
    }
}

impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
    #[inline(always)] fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void {
        let mePtrOfBackgroundSubtractorMOG2 = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBackgroundSubtractorMOG2 as "cv::Ptr<cv::BackgroundSubtractorMOG2>*"] -> *mut c_void as "void*" {
            return mePtrOfBackgroundSubtractorMOG2->get();
        })
    }
}

pub struct PtrOfBaseConvolutionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBaseConvolutionLayer {
    #[inline(always)] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBaseConvolutionLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::BaseConvolutionLayer>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfBaseConvolutionLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfBaseConvolutionLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBaseConvolutionLayer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfBaseConvolutionLayer->get();
        })
    }
}

impl crate::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void {
        let mePtrOfBaseConvolutionLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBaseConvolutionLayer as "cv::Ptr<cv::dnn::BaseConvolutionLayer>*"] -> *mut c_void as "void*" {
            return mePtrOfBaseConvolutionLayer->get();
        })
    }
}

impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
        let mePtrOfBaseConvolutionLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBaseConvolutionLayer as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
            return mePtrOfBaseConvolutionLayer->get();
        })
    }
}

pub struct PtrOfBatchNormLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBatchNormLayer {
    #[inline(always)] pub fn as_raw_PtrOfBatchNormLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBatchNormLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::BatchNormLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBlockMeanHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBlockMeanHash {
    #[inline(always)] pub fn as_raw_PtrOfBlockMeanHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBlockMeanHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::BlockMeanHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfBoost {
    pub(crate) ptr: *mut c_void
}

impl PtrOfBoost {
    #[inline(always)] pub fn as_raw_PtrOfBoost(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfBoost {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::Boost>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfBoost {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfBoost = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBoost as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfBoost->get();
        })
    }
}

impl crate::ml::Boost for PtrOfBoost {
    #[inline(always)] fn as_raw_Boost(&self) -> *mut c_void {
        let mePtrOfBoost = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBoost as "cv::Ptr<cv::ml::Boost>*"] -> *mut c_void as "void*" {
            return mePtrOfBoost->get();
        })
    }
}

impl crate::ml::DTrees for PtrOfBoost {
    #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
        let mePtrOfBoost = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBoost as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
            return mePtrOfBoost->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfBoost {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfBoost = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfBoost as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfBoost->get();
        })
    }
}

pub struct PtrOfCLAHE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCLAHE {
    #[inline(always)] pub fn as_raw_PtrOfCLAHE(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfCLAHE {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::CLAHE>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfCLAHE {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfCLAHE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCLAHE as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfCLAHE->get();
        })
    }
}

impl crate::imgproc::CLAHE for PtrOfCLAHE {
    #[inline(always)] fn as_raw_CLAHE(&self) -> *mut c_void {
        let mePtrOfCLAHE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCLAHE as "cv::Ptr<cv::CLAHE>*"] -> *mut c_void as "void*" {
            return mePtrOfCLAHE->get();
        })
    }
}

pub struct PtrOfCalibrateDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateDebevec {
    #[inline(always)] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfCalibrateDebevec {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::CalibrateDebevec>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfCalibrateDebevec {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfCalibrateDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateDebevec as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateDebevec->get();
        })
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
    #[inline(always)] fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        let mePtrOfCalibrateDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateDebevec as "cv::Ptr<cv::CalibrateCRF>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateDebevec->get();
        })
    }
}

impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
    #[inline(always)] fn as_raw_CalibrateDebevec(&self) -> *mut c_void {
        let mePtrOfCalibrateDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateDebevec as "cv::Ptr<cv::CalibrateDebevec>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateDebevec->get();
        })
    }
}

pub struct PtrOfCalibrateRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCalibrateRobertson {
    #[inline(always)] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfCalibrateRobertson {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::CalibrateRobertson>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfCalibrateRobertson {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfCalibrateRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateRobertson as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateRobertson->get();
        })
    }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
    #[inline(always)] fn as_raw_CalibrateCRF(&self) -> *mut c_void {
        let mePtrOfCalibrateRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateRobertson as "cv::Ptr<cv::CalibrateCRF>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateRobertson->get();
        })
    }
}

impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
    #[inline(always)] fn as_raw_CalibrateRobertson(&self) -> *mut c_void {
        let mePtrOfCalibrateRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfCalibrateRobertson as "cv::Ptr<cv::CalibrateRobertson>*"] -> *mut c_void as "void*" {
            return mePtrOfCalibrateRobertson->get();
        })
    }
}

pub struct PtrOfColorMomentHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfColorMomentHash {
    #[inline(always)] pub fn as_raw_PtrOfColorMomentHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfColorMomentHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::ColorMomentHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfConcatLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConcatLayer {
    #[inline(always)] pub fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfConcatLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ConcatLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfConjGradSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfConjGradSolver {
    #[inline(always)] pub fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfConjGradSolver {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ConjGradSolver>*"] {
            delete me;
        })
    }
}
pub struct PtrOfCropLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfCropLayer {
    #[inline(always)] pub fn as_raw_PtrOfCropLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfCropLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::CropLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfDTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDTrees {
    #[inline(always)] pub fn as_raw_PtrOfDTrees(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDTrees {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::DTrees>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfDTrees {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfDTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDTrees as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfDTrees->get();
        })
    }
}

impl crate::ml::DTrees for PtrOfDTrees {
    #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
        let mePtrOfDTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDTrees as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
            return mePtrOfDTrees->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfDTrees {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfDTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDTrees as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfDTrees->get();
        })
    }
}

pub struct PtrOfDeblurerBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDeblurerBase {
    #[inline(always)] pub fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDeblurerBase {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::DeblurerBase>*"] {
            delete me;
        })
    }
}
impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
    #[inline(always)] fn as_raw_DeblurerBase(&self) -> *mut c_void {
        let mePtrOfDeblurerBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDeblurerBase as "cv::Ptr<cv::videostab::DeblurerBase>*"] -> *mut c_void as "void*" {
            return mePtrOfDeblurerBase->get();
        })
    }
}

pub struct PtrOfDescriptorMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDescriptorMatcher {
    #[inline(always)] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDescriptorMatcher {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::DescriptorMatcher>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfDescriptorMatcher {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfDescriptorMatcher = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDescriptorMatcher as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfDescriptorMatcher->get();
        })
    }
}

impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
    #[inline(always)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
        let mePtrOfDescriptorMatcher = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDescriptorMatcher as "cv::Ptr<cv::DescriptorMatcher>*"] -> *mut c_void as "void*" {
            return mePtrOfDescriptorMatcher->get();
        })
    }
}

pub struct PtrOfDetectionOutputLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDetectionOutputLayer {
    #[inline(always)] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDetectionOutputLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::DetectionOutputLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfDownhillSolver {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDownhillSolver {
    #[inline(always)] pub fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDownhillSolver {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::DownhillSolver>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfDownhillSolver {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfDownhillSolver = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDownhillSolver as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfDownhillSolver->get();
        })
    }
}

impl core::DownhillSolver for PtrOfDownhillSolver {
    #[inline(always)] fn as_raw_DownhillSolver(&self) -> *mut c_void {
        let mePtrOfDownhillSolver = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDownhillSolver as "cv::Ptr<cv::DownhillSolver>*"] -> *mut c_void as "void*" {
            return mePtrOfDownhillSolver->get();
        })
    }
}

impl core::MinProblemSolver for PtrOfDownhillSolver {
    #[inline(always)] fn as_raw_MinProblemSolver(&self) -> *mut c_void {
        let mePtrOfDownhillSolver = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDownhillSolver as "cv::Ptr<cv::MinProblemSolver>*"] -> *mut c_void as "void*" {
            return mePtrOfDownhillSolver->get();
        })
    }
}

pub struct PtrOfDualTVL1OpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfDualTVL1OpticalFlow {
    #[inline(always)] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfDualTVL1OpticalFlow {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::DualTVL1OpticalFlow>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfDualTVL1OpticalFlow {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfDualTVL1OpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDualTVL1OpticalFlow as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfDualTVL1OpticalFlow->get();
        })
    }
}

impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[inline(always)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        let mePtrOfDualTVL1OpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDualTVL1OpticalFlow as "cv::Ptr<cv::DenseOpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfDualTVL1OpticalFlow->get();
        })
    }
}

impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
    #[inline(always)] fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void {
        let mePtrOfDualTVL1OpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfDualTVL1OpticalFlow as "cv::Ptr<cv::DualTVL1OpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfDualTVL1OpticalFlow->get();
        })
    }
}

pub struct PtrOfELULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfELULayer {
    #[inline(always)] pub fn as_raw_PtrOfELULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfELULayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ELULayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfEM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEM {
    #[inline(always)] pub fn as_raw_PtrOfEM(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfEM {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::EM>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfEM {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfEM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfEM as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfEM->get();
        })
    }
}

impl crate::ml::EM for PtrOfEM {
    #[inline(always)] fn as_raw_EM(&self) -> *mut c_void {
        let mePtrOfEM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfEM as "cv::Ptr<cv::ml::EM>*"] -> *mut c_void as "void*" {
            return mePtrOfEM->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfEM {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfEM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfEM as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfEM->get();
        })
    }
}

pub struct PtrOfEltwiseLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfEltwiseLayer {
    #[inline(always)] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfEltwiseLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::EltwiseLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfFarnebackOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFarnebackOpticalFlow {
    #[inline(always)] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFarnebackOpticalFlow {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::FarnebackOpticalFlow>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfFarnebackOpticalFlow {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfFarnebackOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFarnebackOpticalFlow as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfFarnebackOpticalFlow->get();
        })
    }
}

impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[inline(always)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
        let mePtrOfFarnebackOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFarnebackOpticalFlow as "cv::Ptr<cv::DenseOpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfFarnebackOpticalFlow->get();
        })
    }
}

impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
    #[inline(always)] fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void {
        let mePtrOfFarnebackOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFarnebackOpticalFlow as "cv::Ptr<cv::FarnebackOpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfFarnebackOpticalFlow->get();
        })
    }
}

pub struct PtrOfFastFeatureDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFastFeatureDetector {
    #[inline(always)] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFastFeatureDetector {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::FastFeatureDetector>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfFastFeatureDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfFastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFastFeatureDetector as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfFastFeatureDetector->get();
        })
    }
}

impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
    #[inline(always)] fn as_raw_FastFeatureDetector(&self) -> *mut c_void {
        let mePtrOfFastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFastFeatureDetector as "cv::Ptr<cv::FastFeatureDetector>*"] -> *mut c_void as "void*" {
            return mePtrOfFastFeatureDetector->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfFastFeatureDetector {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfFastFeatureDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFastFeatureDetector as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfFastFeatureDetector->get();
        })
    }
}

pub struct PtrOfFeature2D {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFeature2D {
    #[inline(always)] pub fn as_raw_PtrOfFeature2D(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFeature2D {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::Feature2D>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfFeature2D {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfFeature2D = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFeature2D as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfFeature2D->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfFeature2D {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfFeature2D = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFeature2D as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfFeature2D->get();
        })
    }
}

pub struct PtrOfFlannBasedMatcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlannBasedMatcher {
    #[inline(always)] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFlannBasedMatcher {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::FlannBasedMatcher>*"] {
            delete me;
        })
    }
}
pub struct PtrOfFlattenLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFlattenLayer {
    #[inline(always)] pub fn as_raw_PtrOfFlattenLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFlattenLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::FlattenLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfFormatted {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatted {
    #[inline(always)] pub fn as_raw_PtrOfFormatted(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFormatted {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::Formatted>*"] {
            delete me;
        })
    }
}
impl core::Formatted for PtrOfFormatted {
    #[inline(always)] fn as_raw_Formatted(&self) -> *mut c_void {
        let mePtrOfFormatted = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFormatted as "cv::Ptr<cv::Formatted>*"] -> *mut c_void as "void*" {
            return mePtrOfFormatted->get();
        })
    }
}

pub struct PtrOfFormatter {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFormatter {
    #[inline(always)] pub fn as_raw_PtrOfFormatter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFormatter {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::Formatter>*"] {
            delete me;
        })
    }
}
impl core::Formatter for PtrOfFormatter {
    #[inline(always)] fn as_raw_Formatter(&self) -> *mut c_void {
        let mePtrOfFormatter = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFormatter as "cv::Ptr<cv::Formatter>*"] -> *mut c_void as "void*" {
            return mePtrOfFormatter->get();
        })
    }
}

pub struct PtrOfFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFrameSource {
    #[inline(always)] pub fn as_raw_PtrOfFrameSource(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFrameSource {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::superres::FrameSource>*"] {
            delete me;
        })
    }
}
impl crate::superres::FrameSource for PtrOfFrameSource {
    #[inline(always)] fn as_raw_FrameSource(&self) -> *mut c_void {
        let mePtrOfFrameSource = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFrameSource as "cv::Ptr<cv::superres::FrameSource>*"] -> *mut c_void as "void*" {
            return mePtrOfFrameSource->get();
        })
    }
}

pub struct PtrOfFreeType2 {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFreeType2 {
    #[inline(always)] pub fn as_raw_PtrOfFreeType2(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFreeType2 {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::freetype::FreeType2>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfFreeType2 {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfFreeType2 = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFreeType2 as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfFreeType2->get();
        })
    }
}

impl crate::freetype::FreeType2 for PtrOfFreeType2 {
    #[inline(always)] fn as_raw_FreeType2(&self) -> *mut c_void {
        let mePtrOfFreeType2 = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFreeType2 as "cv::Ptr<cv::freetype::FreeType2>*"] -> *mut c_void as "void*" {
            return mePtrOfFreeType2->get();
        })
    }
}

pub struct PtrOfFunction {
    pub(crate) ptr: *mut c_void
}

impl PtrOfFunction {
    #[inline(always)] pub fn as_raw_PtrOfFunction(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfFunction {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::MinProblemSolver::Function>*"] {
            delete me;
        })
    }
}
impl core::MinProblemSolver_Function for PtrOfFunction {
    #[inline(always)] fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void {
        let mePtrOfFunction = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfFunction as "cv::Ptr<cv::MinProblemSolver::Function>*"] -> *mut c_void as "void*" {
            return mePtrOfFunction->get();
        })
    }
}

pub struct PtrOfGFTTDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGFTTDetector {
    #[inline(always)] pub fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfGFTTDetector {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::GFTTDetector>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfGFTTDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfGFTTDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGFTTDetector as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfGFTTDetector->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfGFTTDetector {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfGFTTDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGFTTDetector as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfGFTTDetector->get();
        })
    }
}

impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
    #[inline(always)] fn as_raw_GFTTDetector(&self) -> *mut c_void {
        let mePtrOfGFTTDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGFTTDetector as "cv::Ptr<cv::GFTTDetector>*"] -> *mut c_void as "void*" {
            return mePtrOfGFTTDetector->get();
        })
    }
}

pub struct PtrOfGeneralizedHoughBallard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughBallard {
    #[inline(always)] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfGeneralizedHoughBallard {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::GeneralizedHoughBallard>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughBallard {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughBallard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughBallard as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughBallard->get();
        })
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
    #[inline(always)] fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughBallard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughBallard as "cv::Ptr<cv::GeneralizedHough>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughBallard->get();
        })
    }
}

impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
    #[inline(always)] fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughBallard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughBallard as "cv::Ptr<cv::GeneralizedHoughBallard>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughBallard->get();
        })
    }
}

pub struct PtrOfGeneralizedHoughGuil {
    pub(crate) ptr: *mut c_void
}

impl PtrOfGeneralizedHoughGuil {
    #[inline(always)] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfGeneralizedHoughGuil {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::GeneralizedHoughGuil>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfGeneralizedHoughGuil {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughGuil = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughGuil as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughGuil->get();
        })
    }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
    #[inline(always)] fn as_raw_GeneralizedHough(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughGuil = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughGuil as "cv::Ptr<cv::GeneralizedHough>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughGuil->get();
        })
    }
}

impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
    #[inline(always)] fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void {
        let mePtrOfGeneralizedHoughGuil = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfGeneralizedHoughGuil as "cv::Ptr<cv::GeneralizedHoughGuil>*"] -> *mut c_void as "void*" {
            return mePtrOfGeneralizedHoughGuil->get();
        })
    }
}

pub struct PtrOfHausdorffDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHausdorffDistanceExtractor {
    #[inline(always)] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfHausdorffDistanceExtractor {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::HausdorffDistanceExtractor>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfHausdorffDistanceExtractor {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfHausdorffDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHausdorffDistanceExtractor as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfHausdorffDistanceExtractor->get();
        })
    }
}

impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[inline(always)] fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void {
        let mePtrOfHausdorffDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHausdorffDistanceExtractor as "cv::Ptr<cv::HausdorffDistanceExtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfHausdorffDistanceExtractor->get();
        })
    }
}

impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
    #[inline(always)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        let mePtrOfHausdorffDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHausdorffDistanceExtractor as "cv::Ptr<cv::ShapeDistanceExtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfHausdorffDistanceExtractor->get();
        })
    }
}

pub struct PtrOfHistogramCostExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHistogramCostExtractor {
    #[inline(always)] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfHistogramCostExtractor {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::HistogramCostExtractor>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfHistogramCostExtractor {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfHistogramCostExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHistogramCostExtractor as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfHistogramCostExtractor->get();
        })
    }
}

impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
    #[inline(always)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void {
        let mePtrOfHistogramCostExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHistogramCostExtractor as "cv::Ptr<cv::HistogramCostExtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfHistogramCostExtractor->get();
        })
    }
}

pub struct PtrOfHistogramPhaseUnwrapping {
    pub(crate) ptr: *mut c_void
}

impl PtrOfHistogramPhaseUnwrapping {
    #[inline(always)] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfHistogramPhaseUnwrapping {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfHistogramPhaseUnwrapping {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfHistogramPhaseUnwrapping = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHistogramPhaseUnwrapping as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfHistogramPhaseUnwrapping->get();
        })
    }
}

impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
    #[inline(always)] fn as_raw_HistogramPhaseUnwrapping(&self) -> *mut c_void {
        let mePtrOfHistogramPhaseUnwrapping = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHistogramPhaseUnwrapping as "cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*"] -> *mut c_void as "void*" {
            return mePtrOfHistogramPhaseUnwrapping->get();
        })
    }
}

impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
    #[inline(always)] fn as_raw_PhaseUnwrapping(&self) -> *mut c_void {
        let mePtrOfHistogramPhaseUnwrapping = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfHistogramPhaseUnwrapping as "cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>*"] -> *mut c_void as "void*" {
            return mePtrOfHistogramPhaseUnwrapping->get();
        })
    }
}

pub struct PtrOfIFrameSource {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIFrameSource {
    #[inline(always)] pub fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfIFrameSource {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::IFrameSource>*"] {
            delete me;
        })
    }
}
impl crate::videostab::IFrameSource for PtrOfIFrameSource {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void {
        let mePtrOfIFrameSource = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfIFrameSource as "cv::Ptr<cv::videostab::IFrameSource>*"] -> *mut c_void as "void*" {
            return mePtrOfIFrameSource->get();
        })
    }
}

pub struct PtrOfILog {
    pub(crate) ptr: *mut c_void
}

impl PtrOfILog {
    #[inline(always)] pub fn as_raw_PtrOfILog(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfILog {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::ILog>*"] {
            delete me;
        })
    }
}
impl crate::videostab::ILog for PtrOfILog {
    #[inline(always)] fn as_raw_ILog(&self) -> *mut c_void {
        let mePtrOfILog = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfILog as "cv::Ptr<cv::videostab::ILog>*"] -> *mut c_void as "void*" {
            return mePtrOfILog->get();
        })
    }
}

pub struct PtrOfIMotionStabilizer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfIMotionStabilizer {
    #[inline(always)] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfIMotionStabilizer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::IMotionStabilizer>*"] {
            delete me;
        })
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        let mePtrOfIMotionStabilizer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfIMotionStabilizer as "cv::Ptr<cv::videostab::IMotionStabilizer>*"] -> *mut c_void as "void*" {
            return mePtrOfIMotionStabilizer->get();
        })
    }
}

pub struct PtrOfImageMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfImageMotionEstimatorBase {
    #[inline(always)] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfImageMotionEstimatorBase {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::ImageMotionEstimatorBase>*"] {
            delete me;
        })
    }
}
impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
    #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void {
        let mePtrOfImageMotionEstimatorBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfImageMotionEstimatorBase as "cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*"] -> *mut c_void as "void*" {
            return mePtrOfImageMotionEstimatorBase->get();
        })
    }
}

pub struct PtrOfInnerProductLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInnerProductLayer {
    #[inline(always)] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfInnerProductLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::InnerProductLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfInpainterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfInpainterBase {
    #[inline(always)] pub fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfInpainterBase {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::InpainterBase>*"] {
            delete me;
        })
    }
}
impl crate::videostab::InpainterBase for PtrOfInpainterBase {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void {
        let mePtrOfInpainterBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfInpainterBase as "cv::Ptr<cv::videostab::InpainterBase>*"] -> *mut c_void as "void*" {
            return mePtrOfInpainterBase->get();
        })
    }
}

pub struct PtrOfKAZE {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKAZE {
    #[inline(always)] pub fn as_raw_PtrOfKAZE(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfKAZE {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::KAZE>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfKAZE {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKAZE as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfKAZE->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfKAZE {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKAZE as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfKAZE->get();
        })
    }
}

impl crate::features2d::KAZE for PtrOfKAZE {
    #[inline(always)] fn as_raw_KAZE(&self) -> *mut c_void {
        let mePtrOfKAZE = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKAZE as "cv::Ptr<cv::KAZE>*"] -> *mut c_void as "void*" {
            return mePtrOfKAZE->get();
        })
    }
}

pub struct PtrOfKNearest {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKNearest {
    #[inline(always)] pub fn as_raw_PtrOfKNearest(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfKNearest {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::KNearest>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfKNearest {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfKNearest = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKNearest as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfKNearest->get();
        })
    }
}

impl crate::ml::KNearest for PtrOfKNearest {
    #[inline(always)] fn as_raw_KNearest(&self) -> *mut c_void {
        let mePtrOfKNearest = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKNearest as "cv::Ptr<cv::ml::KNearest>*"] -> *mut c_void as "void*" {
            return mePtrOfKNearest->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfKNearest {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfKNearest = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKNearest as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfKNearest->get();
        })
    }
}

pub struct PtrOfKernel {
    pub(crate) ptr: *mut c_void
}

impl PtrOfKernel {
    #[inline(always)] pub fn as_raw_PtrOfKernel(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfKernel {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::SVM::Kernel>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfKernel {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfKernel = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKernel as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfKernel->get();
        })
    }
}

impl crate::ml::SVM_Kernel for PtrOfKernel {
    #[inline(always)] fn as_raw_SVM_Kernel(&self) -> *mut c_void {
        let mePtrOfKernel = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfKernel as "cv::Ptr<cv::ml::SVM::Kernel>*"] -> *mut c_void as "void*" {
            return mePtrOfKernel->get();
        })
    }
}

pub struct PtrOfLRNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLRNLayer {
    #[inline(always)] pub fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfLRNLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::LRNLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfLSTMLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLSTMLayer {
    #[inline(always)] pub fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfLSTMLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::LSTMLayer>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfLSTMLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfLSTMLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLSTMLayer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfLSTMLayer->get();
        })
    }
}

impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
    #[inline(always)] fn as_raw_LSTMLayer(&self) -> *mut c_void {
        let mePtrOfLSTMLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLSTMLayer as "cv::Ptr<cv::dnn::LSTMLayer>*"] -> *mut c_void as "void*" {
            return mePtrOfLSTMLayer->get();
        })
    }
}

impl crate::dnn::Layer for PtrOfLSTMLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
        let mePtrOfLSTMLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLSTMLayer as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
            return mePtrOfLSTMLayer->get();
        })
    }
}

pub struct PtrOfLineSegmentDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLineSegmentDetector {
    #[inline(always)] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfLineSegmentDetector {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::LineSegmentDetector>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfLineSegmentDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfLineSegmentDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLineSegmentDetector as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfLineSegmentDetector->get();
        })
    }
}

impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
    #[inline(always)] fn as_raw_LineSegmentDetector(&self) -> *mut c_void {
        let mePtrOfLineSegmentDetector = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLineSegmentDetector as "cv::Ptr<cv::LineSegmentDetector>*"] -> *mut c_void as "void*" {
            return mePtrOfLineSegmentDetector->get();
        })
    }
}

pub struct PtrOfLogisticRegression {
    pub(crate) ptr: *mut c_void
}

impl PtrOfLogisticRegression {
    #[inline(always)] pub fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfLogisticRegression {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::LogisticRegression>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfLogisticRegression {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfLogisticRegression = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLogisticRegression as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfLogisticRegression->get();
        })
    }
}

impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
    #[inline(always)] fn as_raw_LogisticRegression(&self) -> *mut c_void {
        let mePtrOfLogisticRegression = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLogisticRegression as "cv::Ptr<cv::ml::LogisticRegression>*"] -> *mut c_void as "void*" {
            return mePtrOfLogisticRegression->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfLogisticRegression {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfLogisticRegression = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfLogisticRegression as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfLogisticRegression->get();
        })
    }
}

pub struct PtrOfMSER {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMSER {
    #[inline(always)] pub fn as_raw_PtrOfMSER(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMSER {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::MSER>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfMSER {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfMSER = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMSER as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfMSER->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfMSER {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfMSER = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMSER as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfMSER->get();
        })
    }
}

impl crate::features2d::MSER for PtrOfMSER {
    #[inline(always)] fn as_raw_MSER(&self) -> *mut c_void {
        let mePtrOfMSER = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMSER as "cv::Ptr<cv::MSER>*"] -> *mut c_void as "void*" {
            return mePtrOfMSER->get();
        })
    }
}

pub struct PtrOfMVNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMVNLayer {
    #[inline(always)] pub fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMVNLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::MVNLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfMarrHildrethHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMarrHildrethHash {
    #[inline(always)] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMarrHildrethHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::MarrHildrethHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfMaskGenerator {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaskGenerator {
    #[inline(always)] pub fn as_raw_PtrOfMaskGenerator(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMaskGenerator {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"] {
            delete me;
        })
    }
}
impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfMaskGenerator {
    #[inline(always)] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void {
        let mePtrOfMaskGenerator = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMaskGenerator as "cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"] -> *mut c_void as "void*" {
            return mePtrOfMaskGenerator->get();
        })
    }
}

pub struct PtrOfMaxUnpoolLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMaxUnpoolLayer {
    #[inline(always)] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMaxUnpoolLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::MaxUnpoolLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfMergeDebevec {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeDebevec {
    #[inline(always)] pub fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMergeDebevec {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::MergeDebevec>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfMergeDebevec {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfMergeDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeDebevec as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeDebevec->get();
        })
    }
}

impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
    #[inline(always)] fn as_raw_MergeDebevec(&self) -> *mut c_void {
        let mePtrOfMergeDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeDebevec as "cv::Ptr<cv::MergeDebevec>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeDebevec->get();
        })
    }
}

impl crate::photo::MergeExposures for PtrOfMergeDebevec {
    #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
        let mePtrOfMergeDebevec = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeDebevec as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeDebevec->get();
        })
    }
}

pub struct PtrOfMergeMertens {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeMertens {
    #[inline(always)] pub fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMergeMertens {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::MergeMertens>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfMergeMertens {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfMergeMertens = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeMertens as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeMertens->get();
        })
    }
}

impl crate::photo::MergeExposures for PtrOfMergeMertens {
    #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
        let mePtrOfMergeMertens = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeMertens as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeMertens->get();
        })
    }
}

impl crate::photo::MergeMertens for PtrOfMergeMertens {
    #[inline(always)] fn as_raw_MergeMertens(&self) -> *mut c_void {
        let mePtrOfMergeMertens = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeMertens as "cv::Ptr<cv::MergeMertens>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeMertens->get();
        })
    }
}

pub struct PtrOfMergeRobertson {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMergeRobertson {
    #[inline(always)] pub fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMergeRobertson {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::MergeRobertson>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfMergeRobertson {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfMergeRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeRobertson as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeRobertson->get();
        })
    }
}

impl crate::photo::MergeExposures for PtrOfMergeRobertson {
    #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
        let mePtrOfMergeRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeRobertson as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeRobertson->get();
        })
    }
}

impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
    #[inline(always)] fn as_raw_MergeRobertson(&self) -> *mut c_void {
        let mePtrOfMergeRobertson = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMergeRobertson as "cv::Ptr<cv::MergeRobertson>*"] -> *mut c_void as "void*" {
            return mePtrOfMergeRobertson->get();
        })
    }
}

pub struct PtrOfMotionEstimatorBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionEstimatorBase {
    #[inline(always)] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMotionEstimatorBase {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::MotionEstimatorBase>*"] {
            delete me;
        })
    }
}
impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
    #[inline(always)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void {
        let mePtrOfMotionEstimatorBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMotionEstimatorBase as "cv::Ptr<cv::videostab::MotionEstimatorBase>*"] -> *mut c_void as "void*" {
            return mePtrOfMotionEstimatorBase->get();
        })
    }
}

pub struct PtrOfMotionFilterBase {
    pub(crate) ptr: *mut c_void
}

impl PtrOfMotionFilterBase {
    #[inline(always)] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfMotionFilterBase {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::videostab::MotionFilterBase>*"] {
            delete me;
        })
    }
}
impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
        let mePtrOfMotionFilterBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMotionFilterBase as "cv::Ptr<cv::videostab::IMotionStabilizer>*"] -> *mut c_void as "void*" {
            return mePtrOfMotionFilterBase->get();
        })
    }
}

impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
    #[inline(always)] fn as_raw_MotionFilterBase(&self) -> *mut c_void {
        let mePtrOfMotionFilterBase = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfMotionFilterBase as "cv::Ptr<cv::videostab::MotionFilterBase>*"] -> *mut c_void as "void*" {
            return mePtrOfMotionFilterBase->get();
        })
    }
}

pub struct PtrOfNormalBayesClassifier {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalBayesClassifier {
    #[inline(always)] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfNormalBayesClassifier {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::NormalBayesClassifier>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfNormalBayesClassifier {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfNormalBayesClassifier = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfNormalBayesClassifier as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfNormalBayesClassifier->get();
        })
    }
}

impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
    #[inline(always)] fn as_raw_NormalBayesClassifier(&self) -> *mut c_void {
        let mePtrOfNormalBayesClassifier = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfNormalBayesClassifier as "cv::Ptr<cv::ml::NormalBayesClassifier>*"] -> *mut c_void as "void*" {
            return mePtrOfNormalBayesClassifier->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfNormalBayesClassifier = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfNormalBayesClassifier as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfNormalBayesClassifier->get();
        })
    }
}

pub struct PtrOfNormalizeBBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfNormalizeBBoxLayer {
    #[inline(always)] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfNormalizeBBoxLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::NormalizeBBoxLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfORB {
    pub(crate) ptr: *mut c_void
}

impl PtrOfORB {
    #[inline(always)] pub fn as_raw_PtrOfORB(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfORB {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ORB>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfORB {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfORB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfORB as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfORB->get();
        })
    }
}

impl crate::features2d::Feature2D for PtrOfORB {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
        let mePtrOfORB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfORB as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
            return mePtrOfORB->get();
        })
    }
}

impl crate::features2d::ORB for PtrOfORB {
    #[inline(always)] fn as_raw_ORB(&self) -> *mut c_void {
        let mePtrOfORB = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfORB as "cv::Ptr<cv::ORB>*"] -> *mut c_void as "void*" {
            return mePtrOfORB->get();
        })
    }
}

pub struct PtrOfPHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPHash {
    #[inline(always)] pub fn as_raw_PtrOfPHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::PHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfPaddingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPaddingLayer {
    #[inline(always)] pub fn as_raw_PtrOfPaddingLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPaddingLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::PaddingLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfParamGrid {
    pub(crate) ptr: *mut c_void
}

impl PtrOfParamGrid {
    #[inline(always)] pub fn as_raw_PtrOfParamGrid(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfParamGrid {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::ParamGrid>*"] {
            delete me;
        })
    }
}
pub struct PtrOfPermuteLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPermuteLayer {
    #[inline(always)] pub fn as_raw_PtrOfPermuteLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPermuteLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::PermuteLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfPlot2d {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPlot2d {
    #[inline(always)] pub fn as_raw_PtrOfPlot2d(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPlot2d {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::plot::Plot2d>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfPlot2d {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfPlot2d = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfPlot2d as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfPlot2d->get();
        })
    }
}

impl crate::plot::Plot2d for PtrOfPlot2d {
    #[inline(always)] fn as_raw_Plot2d(&self) -> *mut c_void {
        let mePtrOfPlot2d = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfPlot2d as "cv::Ptr<cv::plot::Plot2d>*"] -> *mut c_void as "void*" {
            return mePtrOfPlot2d->get();
        })
    }
}

pub struct PtrOfPoolingLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPoolingLayer {
    #[inline(always)] pub fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPoolingLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::PoolingLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfPowerLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPowerLayer {
    #[inline(always)] pub fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPowerLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::PowerLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfPriorBoxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfPriorBoxLayer {
    #[inline(always)] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfPriorBoxLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::PriorBoxLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfProposalLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfProposalLayer {
    #[inline(always)] pub fn as_raw_PtrOfProposalLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfProposalLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ProposalLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfRNNLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRNNLayer {
    #[inline(always)] pub fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRNNLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::RNNLayer>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfRNNLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfRNNLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRNNLayer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfRNNLayer->get();
        })
    }
}

impl crate::dnn::Layer for PtrOfRNNLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
        let mePtrOfRNNLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRNNLayer as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
            return mePtrOfRNNLayer->get();
        })
    }
}

impl crate::dnn::RNNLayer for PtrOfRNNLayer {
    #[inline(always)] fn as_raw_RNNLayer(&self) -> *mut c_void {
        let mePtrOfRNNLayer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRNNLayer as "cv::Ptr<cv::dnn::RNNLayer>*"] -> *mut c_void as "void*" {
            return mePtrOfRNNLayer->get();
        })
    }
}

pub struct PtrOfRTrees {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRTrees {
    #[inline(always)] pub fn as_raw_PtrOfRTrees(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRTrees {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::RTrees>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfRTrees {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfRTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRTrees as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfRTrees->get();
        })
    }
}

impl crate::ml::DTrees for PtrOfRTrees {
    #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
        let mePtrOfRTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRTrees as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
            return mePtrOfRTrees->get();
        })
    }
}

impl crate::ml::RTrees for PtrOfRTrees {
    #[inline(always)] fn as_raw_RTrees(&self) -> *mut c_void {
        let mePtrOfRTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRTrees as "cv::Ptr<cv::ml::RTrees>*"] -> *mut c_void as "void*" {
            return mePtrOfRTrees->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfRTrees {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfRTrees = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRTrees as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfRTrees->get();
        })
    }
}

pub struct PtrOfRadialVarianceHash {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRadialVarianceHash {
    #[inline(always)] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRadialVarianceHash {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::img_hash::RadialVarianceHash>*"] {
            delete me;
        })
    }
}
pub struct PtrOfReLU6Layer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLU6Layer {
    #[inline(always)] pub fn as_raw_PtrOfReLU6Layer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfReLU6Layer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ReLU6Layer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfReLULayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReLULayer {
    #[inline(always)] pub fn as_raw_PtrOfReLULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfReLULayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ReLULayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfRegionLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRegionLayer {
    #[inline(always)] pub fn as_raw_PtrOfRegionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRegionLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::RegionLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfReorgLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReorgLayer {
    #[inline(always)] pub fn as_raw_PtrOfReorgLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfReorgLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ReorgLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfReshapeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfReshapeLayer {
    #[inline(always)] pub fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfReshapeLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ReshapeLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfResizeLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfResizeLayer {
    #[inline(always)] pub fn as_raw_PtrOfResizeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfResizeLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ResizeLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfRetina {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRetina {
    #[inline(always)] pub fn as_raw_PtrOfRetina(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRetina {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::bioinspired::Retina>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfRetina {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfRetina = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRetina as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfRetina->get();
        })
    }
}

impl crate::bioinspired::Retina for PtrOfRetina {
    #[inline(always)] fn as_raw_Retina(&self) -> *mut c_void {
        let mePtrOfRetina = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRetina as "cv::Ptr<cv::bioinspired::Retina>*"] -> *mut c_void as "void*" {
            return mePtrOfRetina->get();
        })
    }
}

pub struct PtrOfRetinaFastToneMapping {
    pub(crate) ptr: *mut c_void
}

impl PtrOfRetinaFastToneMapping {
    #[inline(always)] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfRetinaFastToneMapping {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::bioinspired::RetinaFastToneMapping>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfRetinaFastToneMapping {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfRetinaFastToneMapping = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRetinaFastToneMapping as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfRetinaFastToneMapping->get();
        })
    }
}

impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
    #[inline(always)] fn as_raw_RetinaFastToneMapping(&self) -> *mut c_void {
        let mePtrOfRetinaFastToneMapping = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfRetinaFastToneMapping as "cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*"] -> *mut c_void as "void*" {
            return mePtrOfRetinaFastToneMapping->get();
        })
    }
}

pub struct PtrOfSVM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVM {
    #[inline(always)] pub fn as_raw_PtrOfSVM(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSVM {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::SVM>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfSVM {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfSVM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVM as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfSVM->get();
        })
    }
}

impl crate::ml::SVM for PtrOfSVM {
    #[inline(always)] fn as_raw_SVM(&self) -> *mut c_void {
        let mePtrOfSVM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVM as "cv::Ptr<cv::ml::SVM>*"] -> *mut c_void as "void*" {
            return mePtrOfSVM->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfSVM {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfSVM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVM as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfSVM->get();
        })
    }
}

pub struct PtrOfSVMSGD {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSVMSGD {
    #[inline(always)] pub fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSVMSGD {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::SVMSGD>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfSVMSGD {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfSVMSGD = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVMSGD as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfSVMSGD->get();
        })
    }
}

impl crate::ml::SVMSGD for PtrOfSVMSGD {
    #[inline(always)] fn as_raw_SVMSGD(&self) -> *mut c_void {
        let mePtrOfSVMSGD = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVMSGD as "cv::Ptr<cv::ml::SVMSGD>*"] -> *mut c_void as "void*" {
            return mePtrOfSVMSGD->get();
        })
    }
}

impl crate::ml::StatModel for PtrOfSVMSGD {
    #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
        let mePtrOfSVMSGD = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSVMSGD as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
            return mePtrOfSVMSGD->get();
        })
    }
}

pub struct PtrOfScaleLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfScaleLayer {
    #[inline(always)] pub fn as_raw_PtrOfScaleLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfScaleLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::ScaleLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfShapeContextDistanceExtractor {
    pub(crate) ptr: *mut c_void
}

impl PtrOfShapeContextDistanceExtractor {
    #[inline(always)] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfShapeContextDistanceExtractor {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ShapeContextDistanceExtractor>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfShapeContextDistanceExtractor {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfShapeContextDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfShapeContextDistanceExtractor as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfShapeContextDistanceExtractor->get();
        })
    }
}

impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[inline(always)] fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void {
        let mePtrOfShapeContextDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfShapeContextDistanceExtractor as "cv::Ptr<cv::ShapeContextDistanceExtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfShapeContextDistanceExtractor->get();
        })
    }
}

impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
    #[inline(always)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
        let mePtrOfShapeContextDistanceExtractor = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfShapeContextDistanceExtractor as "cv::Ptr<cv::ShapeDistanceExtractor>*"] -> *mut c_void as "void*" {
            return mePtrOfShapeContextDistanceExtractor->get();
        })
    }
}

pub struct PtrOfSigmoidLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSigmoidLayer {
    #[inline(always)] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSigmoidLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::SigmoidLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfSimpleBlobDetector {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSimpleBlobDetector {
    #[inline(always)] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSimpleBlobDetector {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::SimpleBlobDetector>*"] {
            delete me;
        })
    }
}
pub struct PtrOfSliceLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSliceLayer {
    #[inline(always)] pub fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSliceLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::SliceLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfSoftmaxLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSoftmaxLayer {
    #[inline(always)] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSoftmaxLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::SoftmaxLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfSparsePyrLKOpticalFlow {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSparsePyrLKOpticalFlow {
    #[inline(always)] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSparsePyrLKOpticalFlow {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::SparsePyrLKOpticalFlow>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfSparsePyrLKOpticalFlow {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfSparsePyrLKOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSparsePyrLKOpticalFlow as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfSparsePyrLKOpticalFlow->get();
        })
    }
}

impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[inline(always)] fn as_raw_SparseOpticalFlow(&self) -> *mut c_void {
        let mePtrOfSparsePyrLKOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSparsePyrLKOpticalFlow as "cv::Ptr<cv::SparseOpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfSparsePyrLKOpticalFlow->get();
        })
    }
}

impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
    #[inline(always)] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void {
        let mePtrOfSparsePyrLKOpticalFlow = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSparsePyrLKOpticalFlow as "cv::Ptr<cv::SparsePyrLKOpticalFlow>*"] -> *mut c_void as "void*" {
            return mePtrOfSparsePyrLKOpticalFlow->get();
        })
    }
}

pub struct PtrOfSplitLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSplitLayer {
    #[inline(always)] pub fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSplitLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::SplitLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfStereoBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoBM {
    #[inline(always)] pub fn as_raw_PtrOfStereoBM(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfStereoBM {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::StereoBM>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfStereoBM {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfStereoBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoBM as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoBM->get();
        })
    }
}

impl crate::calib3d::StereoBM for PtrOfStereoBM {
    #[inline(always)] fn as_raw_StereoBM(&self) -> *mut c_void {
        let mePtrOfStereoBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoBM as "cv::Ptr<cv::StereoBM>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoBM->get();
        })
    }
}

impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
    #[inline(always)] fn as_raw_StereoMatcher(&self) -> *mut c_void {
        let mePtrOfStereoBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoBM as "cv::Ptr<cv::StereoMatcher>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoBM->get();
        })
    }
}

pub struct PtrOfStereoSGBM {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStereoSGBM {
    #[inline(always)] pub fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfStereoSGBM {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::StereoSGBM>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfStereoSGBM {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfStereoSGBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoSGBM as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoSGBM->get();
        })
    }
}

impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
    #[inline(always)] fn as_raw_StereoMatcher(&self) -> *mut c_void {
        let mePtrOfStereoSGBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoSGBM as "cv::Ptr<cv::StereoMatcher>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoSGBM->get();
        })
    }
}

impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
    #[inline(always)] fn as_raw_StereoSGBM(&self) -> *mut c_void {
        let mePtrOfStereoSGBM = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfStereoSGBM as "cv::Ptr<cv::StereoSGBM>*"] -> *mut c_void as "void*" {
            return mePtrOfStereoSGBM->get();
        })
    }
}

pub struct PtrOfStitcher {
    pub(crate) ptr: *mut c_void
}

impl PtrOfStitcher {
    #[inline(always)] pub fn as_raw_PtrOfStitcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfStitcher {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::Stitcher>*"] {
            delete me;
        })
    }
}
pub struct PtrOfSuperResolution {
    pub(crate) ptr: *mut c_void
}

impl PtrOfSuperResolution {
    #[inline(always)] pub fn as_raw_PtrOfSuperResolution(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfSuperResolution {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::superres::SuperResolution>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfSuperResolution {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfSuperResolution = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSuperResolution as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfSuperResolution->get();
        })
    }
}

impl crate::superres::FrameSource for PtrOfSuperResolution {
    #[inline(always)] fn as_raw_FrameSource(&self) -> *mut c_void {
        let mePtrOfSuperResolution = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSuperResolution as "cv::Ptr<cv::superres::FrameSource>*"] -> *mut c_void as "void*" {
            return mePtrOfSuperResolution->get();
        })
    }
}

impl crate::superres::SuperResolution for PtrOfSuperResolution {
    #[inline(always)] fn as_raw_SuperResolution(&self) -> *mut c_void {
        let mePtrOfSuperResolution = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfSuperResolution as "cv::Ptr<cv::superres::SuperResolution>*"] -> *mut c_void as "void*" {
            return mePtrOfSuperResolution->get();
        })
    }
}

pub struct PtrOfTanHLayer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTanHLayer {
    #[inline(always)] pub fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTanHLayer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::dnn::TanHLayer>*"] {
            delete me;
        })
    }
}
pub struct PtrOfThinPlateSplineShapeTransformer {
    pub(crate) ptr: *mut c_void
}

impl PtrOfThinPlateSplineShapeTransformer {
    #[inline(always)] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfThinPlateSplineShapeTransformer {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ThinPlateSplineShapeTransformer>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfThinPlateSplineShapeTransformer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfThinPlateSplineShapeTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfThinPlateSplineShapeTransformer as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfThinPlateSplineShapeTransformer->get();
        })
    }
}

impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[inline(always)] fn as_raw_ShapeTransformer(&self) -> *mut c_void {
        let mePtrOfThinPlateSplineShapeTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfThinPlateSplineShapeTransformer as "cv::Ptr<cv::ShapeTransformer>*"] -> *mut c_void as "void*" {
            return mePtrOfThinPlateSplineShapeTransformer->get();
        })
    }
}

impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
    #[inline(always)] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void {
        let mePtrOfThinPlateSplineShapeTransformer = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfThinPlateSplineShapeTransformer as "cv::Ptr<cv::ThinPlateSplineShapeTransformer>*"] -> *mut c_void as "void*" {
            return mePtrOfThinPlateSplineShapeTransformer->get();
        })
    }
}

pub struct PtrOfTonemap {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemap {
    #[inline(always)] pub fn as_raw_PtrOfTonemap(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTonemap {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::Tonemap>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfTonemap {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfTonemap = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemap as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemap->get();
        })
    }
}

impl crate::photo::Tonemap for PtrOfTonemap {
    #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
        let mePtrOfTonemap = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemap as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemap->get();
        })
    }
}

pub struct PtrOfTonemapDrago {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapDrago {
    #[inline(always)] pub fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTonemapDrago {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::TonemapDrago>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfTonemapDrago {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfTonemapDrago = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapDrago as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapDrago->get();
        })
    }
}

impl crate::photo::Tonemap for PtrOfTonemapDrago {
    #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
        let mePtrOfTonemapDrago = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapDrago as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapDrago->get();
        })
    }
}

impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
    #[inline(always)] fn as_raw_TonemapDrago(&self) -> *mut c_void {
        let mePtrOfTonemapDrago = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapDrago as "cv::Ptr<cv::TonemapDrago>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapDrago->get();
        })
    }
}

pub struct PtrOfTonemapMantiuk {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapMantiuk {
    #[inline(always)] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTonemapMantiuk {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::TonemapMantiuk>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfTonemapMantiuk {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfTonemapMantiuk = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapMantiuk as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapMantiuk->get();
        })
    }
}

impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
    #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
        let mePtrOfTonemapMantiuk = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapMantiuk as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapMantiuk->get();
        })
    }
}

impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
    #[inline(always)] fn as_raw_TonemapMantiuk(&self) -> *mut c_void {
        let mePtrOfTonemapMantiuk = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapMantiuk as "cv::Ptr<cv::TonemapMantiuk>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapMantiuk->get();
        })
    }
}

pub struct PtrOfTonemapReinhard {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTonemapReinhard {
    #[inline(always)] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTonemapReinhard {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::TonemapReinhard>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfTonemapReinhard {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfTonemapReinhard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapReinhard as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapReinhard->get();
        })
    }
}

impl crate::photo::Tonemap for PtrOfTonemapReinhard {
    #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
        let mePtrOfTonemapReinhard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapReinhard as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapReinhard->get();
        })
    }
}

impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
    #[inline(always)] fn as_raw_TonemapReinhard(&self) -> *mut c_void {
        let mePtrOfTonemapReinhard = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTonemapReinhard as "cv::Ptr<cv::TonemapReinhard>*"] -> *mut c_void as "void*" {
            return mePtrOfTonemapReinhard->get();
        })
    }
}

pub struct PtrOfTrainData {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTrainData {
    #[inline(always)] pub fn as_raw_PtrOfTrainData(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTrainData {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::ml::TrainData>*"] {
            delete me;
        })
    }
}
impl crate::ml::TrainData for PtrOfTrainData {
    #[inline(always)] fn as_raw_TrainData(&self) -> *mut c_void {
        let mePtrOfTrainData = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTrainData as "cv::Ptr<cv::ml::TrainData>*"] -> *mut c_void as "void*" {
            return mePtrOfTrainData->get();
        })
    }
}

pub struct PtrOfTransientAreasSegmentationModule {
    pub(crate) ptr: *mut c_void
}

impl PtrOfTransientAreasSegmentationModule {
    #[inline(always)] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOfTransientAreasSegmentationModule {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<cv::bioinspired::TransientAreasSegmentationModule>*"] {
            delete me;
        })
    }
}
impl core::Algorithm for PtrOfTransientAreasSegmentationModule {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
        let mePtrOfTransientAreasSegmentationModule = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTransientAreasSegmentationModule as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
            return mePtrOfTransientAreasSegmentationModule->get();
        })
    }
}

impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
    #[inline(always)] fn as_raw_TransientAreasSegmentationModule(&self) -> *mut c_void {
        let mePtrOfTransientAreasSegmentationModule = self.ptr; // weird var name is needed to prevent rust-cpp from generating functions with identical names
        cpp!(unsafe [mePtrOfTransientAreasSegmentationModule as "cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*"] -> *mut c_void as "void*" {
            return mePtrOfTransientAreasSegmentationModule->get();
        })
    }
}

pub struct PtrOffloat {
    pub(crate) ptr: *mut c_void
}

impl PtrOffloat {
    #[inline(always)] pub fn as_raw_PtrOffloat(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl Drop for PtrOffloat {
    fn drop(&mut self) {
        let me = self.ptr;
        cpp!(unsafe [me as "Ptr<float>*"] {
            delete me;
        })
    }
}
pub struct VectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

impl VectorOfDMatch {
    #[inline(always)] pub fn as_raw_VectorOfDMatch(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::DMatch] {
        unsafe {
            let vec_d = self.as_raw_VectorOfDMatch();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::DMatch>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::DMatch>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfDMatch {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfDMatch {
    type Item = core::DMatch;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfDMatch {
    type Item = core::DMatch;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDMatch>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfDMatch {
    type Storage = core::DMatch;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::DMatch>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
            vec->clear();
        })
    }

    type Arg = core::DMatch;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", val as "cv::DMatch"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<cv::DMatch>*", index as "size_t"] -> crate::sys::cv_return_value_DMatch as "cv_return_value_DMatch" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_DMatch)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec_unchkd as "const std::vector<cv::DMatch>*", index as "size_t"] -> core::DMatch as "cv::DMatch" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfDetectionROI {
    pub(crate) ptr: *mut c_void
}

impl VectorOfDetectionROI {
    #[inline(always)] pub fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfDetectionROI {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfDetectionROI {
    type Item = crate::objdetect::DetectionROI;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfDetectionROI {
    type Item = crate::objdetect::DetectionROI;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetectionROI>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfDetectionROI {
    type Storage = crate::objdetect::DetectionROI;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::DetectionROI>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
            vec->clear();
        })
    }

    type Arg = crate::objdetect::DetectionROI;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfDetectionROI();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", val as "cv::DetectionROI*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfDetectionROI();
        let val = val.as_raw_DetectionROI();
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfDetectionROI();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::DetectionROI(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| crate::objdetect::DetectionROI { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfDetectionROI();
        crate::objdetect::DetectionROI { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::DetectionROI>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::DetectionROI((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfDetectionROI();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfDetectionROI();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfExtObject {
    pub(crate) ptr: *mut c_void
}

impl VectorOfExtObject {
    #[inline(always)] pub fn as_raw_VectorOfExtObject(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfExtObject {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfExtObject {
    type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfExtObject {
    type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfExtObject>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfExtObject {
    type Storage = crate::objdetect::DetectionBasedTracker_ExtObject;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::DetectionBasedTracker::ExtObject>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
            vec->clear();
        })
    }

    type Arg = crate::objdetect::DetectionBasedTracker_ExtObject;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfExtObject();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", val as "cv::DetectionBasedTracker::ExtObject*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfExtObject();
        let val = val.as_raw_DetectionBasedTracker_ExtObject();
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfExtObject();
        cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::DetectionBasedTracker::ExtObject(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| crate::objdetect::DetectionBasedTracker_ExtObject { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfExtObject();
        crate::objdetect::DetectionBasedTracker_ExtObject { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::DetectionBasedTracker::ExtObject((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfExtObject();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfExtObject();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfKeyPoint {
    #[inline(always)] pub fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::KeyPoint] {
        unsafe {
            let vec_d = self.as_raw_VectorOfKeyPoint();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::KeyPoint>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::KeyPoint>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfKeyPoint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfKeyPoint {
    type Item = core::KeyPoint;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfKeyPoint {
    type Item = core::KeyPoint;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfKeyPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfKeyPoint {
    type Storage = core::KeyPoint;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::KeyPoint>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
            vec->clear();
        })
    }

    type Arg = core::KeyPoint;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", val as "cv::KeyPoint"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*", index as "size_t"] -> crate::sys::cv_return_value_KeyPoint as "cv_return_value_KeyPoint" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_KeyPoint)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec_unchkd as "const std::vector<cv::KeyPoint>*", index as "size_t"] -> core::KeyPoint as "cv::KeyPoint" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfMat {
    pub(crate) ptr: *mut c_void
}

impl VectorOfMat {
    #[inline(always)] pub fn as_raw_VectorOfMat(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfMat {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfMat {
    type Item = core::Mat;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfMat {
    type Item = core::Mat;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfMat>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfMat {
    type Storage = core::Mat;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Mat>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
            vec->clear();
        })
    }

    type Arg = core::Mat;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfMat();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", val as "cv::Mat*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfMat();
        let val = val.as_raw_Mat();
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "const std::vector<cv::Mat>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::Mat(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| core::Mat { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfMat();
        core::Mat { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::Mat>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::Mat((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfMat();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfMat();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfNode {
    pub(crate) ptr: *mut c_void
}

impl VectorOfNode {
    #[inline(always)] pub fn as_raw_VectorOfNode(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfNode {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfNode {
    type Item = crate::ml::DTrees_Node;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfNode {
    type Item = crate::ml::DTrees_Node;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfNode>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfNode {
    type Storage = crate::ml::DTrees_Node;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::ml::DTrees::Node>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
            vec->clear();
        })
    }

    type Arg = crate::ml::DTrees_Node;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfNode();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", val as "cv::ml::DTrees::Node*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfNode();
        let val = val.as_raw_DTrees_Node();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfNode();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::ml::DTrees::Node(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| crate::ml::DTrees_Node { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfNode();
        crate::ml::DTrees_Node { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::ml::DTrees::Node>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::ml::DTrees::Node((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfNode();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfNode();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfObjectDetection {
    pub(crate) ptr: *mut c_void
}

impl VectorOfObjectDetection {
    #[inline(always)] pub fn as_raw_VectorOfObjectDetection(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfObjectDetection {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfObjectDetection {
    type Item = crate::dpm::DPMDetector_ObjectDetection;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfObjectDetection {
    type Item = crate::dpm::DPMDetector_ObjectDetection;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfObjectDetection>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfObjectDetection {
    type Storage = crate::dpm::DPMDetector_ObjectDetection;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::dpm::DPMDetector::ObjectDetection>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
            vec->clear();
        })
    }

    type Arg = crate::dpm::DPMDetector_ObjectDetection;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfObjectDetection();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfObjectDetection();
        let val = val.as_raw_DPMDetector_ObjectDetection();
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfObjectDetection();
        cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::dpm::DPMDetector::ObjectDetection(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfObjectDetection();
        crate::dpm::DPMDetector_ObjectDetection { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::dpm::DPMDetector::ObjectDetection((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfObjectDetection();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfObjectDetection();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint {
    #[inline(always)] pub fn as_raw_VectorOfPoint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Point] {
        unsafe {
            let vec_d = self.as_raw_VectorOfPoint();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Point>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Point>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfPoint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfPoint {
    type Item = core::Point;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfPoint {
    type Item = core::Point;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfPoint {
    type Storage = core::Point;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Point>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
            vec->clear();
        })
    }

    type Arg = core::Point;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", val as "cv::Point"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<PointWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_PointWrapper as "cv_return_value_PointWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_PointWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec_unchkd as "const std::vector<PointWrapper>*", index as "size_t"] -> core::Point as "PointWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfPoint2d {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint2d {
    #[inline(always)] pub fn as_raw_VectorOfPoint2d(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Point2d] {
        unsafe {
            let vec_d = self.as_raw_VectorOfPoint2d();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Point2d>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Point2d>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfPoint2d {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfPoint2d {
    type Item = core::Point2d;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfPoint2d {
    type Item = core::Point2d;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2d>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfPoint2d {
    type Storage = core::Point2d;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Point2d>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
            vec->clear();
        })
    }

    type Arg = core::Point2d;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", val as "cv::Point2d"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "const std::vector<Point2dWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point2dWrapper as "cv_return_value_Point2dWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_Point2dWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec_unchkd as "const std::vector<Point2dWrapper>*", index as "size_t"] -> core::Point2d as "Point2dWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint2d();
        cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPoint2f {
    #[inline(always)] pub fn as_raw_VectorOfPoint2f(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Point2f] {
        unsafe {
            let vec_d = self.as_raw_VectorOfPoint2f();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Point2f>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Point2f>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfPoint2f {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfPoint2f {
    type Item = core::Point2f;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfPoint2f {
    type Item = core::Point2f;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2f>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfPoint2f {
    type Storage = core::Point2f;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Point2f>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
            vec->clear();
        })
    }

    type Arg = core::Point2f;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", val as "cv::Point2f"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<Point2fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point2fWrapper as "cv_return_value_Point2fWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_Point2fWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec_unchkd as "const std::vector<Point2fWrapper>*", index as "size_t"] -> core::Point2f as "Point2fWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfPtrOfBackendWrapper {
    pub(crate) ptr: *mut c_void
}

impl VectorOfPtrOfBackendWrapper {
    #[inline(always)] pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfPtrOfBackendWrapper {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfPtrOfBackendWrapper {
    type Item = types::PtrOfBackendWrapper;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfPtrOfBackendWrapper {
    type Item = types::PtrOfBackendWrapper;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPtrOfBackendWrapper>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfPtrOfBackendWrapper {
    type Storage = types::PtrOfBackendWrapper;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<Ptr<cv::dnn::BackendWrapper>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "const std::vector<Ptr<cv::dnn::BackendWrapper>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "const std::vector<Ptr<cv::dnn::BackendWrapper>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "const std::vector<Ptr<cv::dnn::BackendWrapper>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*"] {
            vec->clear();
        })
    }

    type Arg = types::PtrOfBackendWrapper;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", val as "Ptr<cv::dnn::BackendWrapper>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        let val = val.as_raw_PtrOfBackendWrapper();
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t", val as "Ptr<cv::dnn::BackendWrapper>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        cpp!(unsafe [vec as "const std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new Ptr<cv::dnn::BackendWrapper>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::PtrOfBackendWrapper { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfPtrOfBackendWrapper();
        types::PtrOfBackendWrapper { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new Ptr<cv::dnn::BackendWrapper>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t", val as "Ptr<cv::dnn::BackendWrapper>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfPtrOfBackendWrapper();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<Ptr<cv::dnn::BackendWrapper>>*", index as "size_t", val as "Ptr<cv::dnn::BackendWrapper>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfRange {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRange {
    #[inline(always)] pub fn as_raw_VectorOfRange(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfRange {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfRange {
    type Item = core::Range;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfRange {
    type Item = core::Range;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRange>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfRange {
    type Storage = core::Range;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Range>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "std::vector<cv::Range>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
            vec->clear();
        })
    }

    type Arg = core::Range;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRange();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::Range>*", val as "cv::Range*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfRange();
        let val = val.as_raw_Range();
        cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfRange();
        cpp!(unsafe [vec as "const std::vector<cv::Range>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::Range(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| core::Range { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfRange();
        core::Range { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::Range>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::Range((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfRange();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRange();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRect {
    #[inline(always)] pub fn as_raw_VectorOfRect(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Rect] {
        unsafe {
            let vec_d = self.as_raw_VectorOfRect();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Rect>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Rect>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfRect {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfRect {
    type Item = core::Rect;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfRect {
    type Item = core::Rect;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRect>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfRect {
    type Storage = core::Rect;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Rect>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
            vec->clear();
        })
    }

    type Arg = core::Rect;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", val as "cv::Rect"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "const std::vector<RectWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_RectWrapper as "cv_return_value_RectWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_RectWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec_unchkd as "const std::vector<RectWrapper>*", index as "size_t"] -> core::Rect as "RectWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfRect2d {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRect2d {
    #[inline(always)] pub fn as_raw_VectorOfRect2d(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Rect2d] {
        unsafe {
            let vec_d = self.as_raw_VectorOfRect2d();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Rect2d>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Rect2d>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfRect2d {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfRect2d {
    type Item = core::Rect2d;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfRect2d {
    type Item = core::Rect2d;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRect2d>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfRect2d {
    type Storage = core::Rect2d;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Rect2d>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "const std::vector<cv::Rect2d>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "const std::vector<cv::Rect2d>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "const std::vector<cv::Rect2d>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*"] {
            vec->clear();
        })
    }

    type Arg = core::Rect2d;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", val as "cv::Rect2d"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", index as "size_t", val as "cv::Rect2d"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "const std::vector<Rect2dWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Rect2dWrapper as "cv_return_value_Rect2dWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_Rect2dWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec_unchkd as "const std::vector<Rect2dWrapper>*", index as "size_t"] -> core::Rect2d as "Rect2dWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", index as "size_t", val as "cv::Rect2d"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRect2d();
        cpp!(unsafe [vec as "std::vector<cv::Rect2d>*", index as "size_t", val as "cv::Rect2d"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfRotatedRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfRotatedRect {
    #[inline(always)] pub fn as_raw_VectorOfRotatedRect(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfRotatedRect {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfRotatedRect {
    type Item = core::RotatedRect;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfRotatedRect {
    type Item = core::RotatedRect;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRotatedRect>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfRotatedRect {
    type Storage = core::RotatedRect;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::RotatedRect>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "const std::vector<cv::RotatedRect>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "const std::vector<cv::RotatedRect>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "const std::vector<cv::RotatedRect>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*"] {
            vec->clear();
        })
    }

    type Arg = core::RotatedRect;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRotatedRect();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", val as "cv::RotatedRect*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfRotatedRect();
        let val = val.as_raw_RotatedRect();
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", index as "size_t", val as "cv::RotatedRect*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfRotatedRect();
        cpp!(unsafe [vec as "const std::vector<cv::RotatedRect>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::RotatedRect(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| core::RotatedRect { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfRotatedRect();
        core::RotatedRect { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::RotatedRect>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::RotatedRect((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfRotatedRect();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", index as "size_t", val as "cv::RotatedRect*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfRotatedRect();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::RotatedRect>*", index as "size_t", val as "cv::RotatedRect*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfSplit {
    pub(crate) ptr: *mut c_void
}

impl VectorOfSplit {
    #[inline(always)] pub fn as_raw_VectorOfSplit(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfSplit {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfSplit {
    type Item = crate::ml::DTrees_Split;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfSplit {
    type Item = crate::ml::DTrees_Split;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfSplit>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfSplit {
    type Storage = crate::ml::DTrees_Split;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::ml::DTrees::Split>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
            vec->clear();
        })
    }

    type Arg = crate::ml::DTrees_Split;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfSplit();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", val as "cv::ml::DTrees::Split*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfSplit();
        let val = val.as_raw_DTrees_Split();
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfSplit();
        cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new cv::ml::DTrees::Split(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| crate::ml::DTrees_Split { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfSplit();
        crate::ml::DTrees_Split { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<cv::ml::DTrees::Split>*", index as "size_t"] -> *mut c_void as "void*" {
            return new cv::ml::DTrees::Split((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfSplit();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfSplit();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfString {
    pub(crate) ptr: *mut c_void
}

impl VectorOfString {
    #[inline(always)] pub fn as_raw_VectorOfString(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfString {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "std::vector<String>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfString {
    type Item = String;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfString {
    type Item = String;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfString {
    type Storage = String;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<String>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "const std::vector<String>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "std::vector<String>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "std::vector<String>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "std::vector<String>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "std::vector<String>*"] {
            vec->clear();
        })
    }

    type Arg = &'i str;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfString();
        let val = ::std::ffi::CString::new(val).unwrap();
        let val = val.as_ptr();
        cpp!(unsafe [vec as "std::vector<String>*", val as "const char*"] {
            vec->push_back(String(val));
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfString();
        let val = ::std::ffi::CString::new(val).unwrap();
        let val = val.as_ptr();
        cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
            vec->insert(vec->begin() + index, String(val));
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfString();
        cpp!(unsafe [vec as "const std::vector<String>*", index as "size_t"] -> crate::sys::cv_return_value_char_X as "cv_return_value_char_X" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index).c_str() };
            } VEC_CATCH(cv_return_value_char_X)
        }).into_result().map(|x| unsafe { ::std::ffi::CStr::from_ptr(x) }.to_string_lossy().into_owned())
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfString();
        ::std::ffi::CStr::from_ptr(cpp!(unsafe [vec_unchkd as "const std::vector<String>*", index as "size_t"] -> *mut c_char as "const char*" {
            return (*vec_unchkd)[index].c_str();
        })).to_string_lossy().into_owned()
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfString();
        let val = ::std::ffi::CString::new(val).unwrap();
        let val = val.as_ptr();
        cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = String(val);
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfString();
        let val = ::std::ffi::CString::new(val).unwrap();
        let val = val.as_ptr();
        cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
            (*vec)[index] = String(val);
        })
    }
}

pub struct VectorOfVec4f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVec4f {
    #[inline(always)] pub fn as_raw_VectorOfVec4f(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Vec4f] {
        unsafe {
            let vec_d = self.as_raw_VectorOfVec4f();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Vec4f>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Vec4f>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfVec4f {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVec4f {
    type Item = core::Vec4f;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVec4f {
    type Item = core::Vec4f;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec4f>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVec4f {
    type Storage = core::Vec4f;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Vec4f>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
            vec->clear();
        })
    }

    type Arg = core::Vec4f;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", val as "cv::Vec4f"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "const std::vector<Vec4fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Vec4fWrapper as "cv_return_value_Vec4fWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_Vec4fWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec_unchkd as "const std::vector<Vec4fWrapper>*", index as "size_t"] -> core::Vec4f as "Vec4fWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVec4f();
        cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfVec6f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVec6f {
    #[inline(always)] pub fn as_raw_VectorOfVec6f(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[core::Vec6f] {
        unsafe {
            let vec_d = self.as_raw_VectorOfVec6f();
            let data = cpp!(unsafe [vec_d as "std::vector<cv::Vec6f>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<cv::Vec6f>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfVec6f {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVec6f {
    type Item = core::Vec6f;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVec6f {
    type Item = core::Vec6f;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec6f>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVec6f {
    type Storage = core::Vec6f;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<cv::Vec6f>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
            vec->clear();
        })
    }

    type Arg = core::Vec6f;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", val as "cv::Vec6f"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "const std::vector<Vec6fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Vec6fWrapper as "cv_return_value_Vec6fWrapper" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_Vec6fWrapper)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec_unchkd as "const std::vector<Vec6fWrapper>*", index as "size_t"] -> core::Vec6f as "Vec6fWrapper" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVec6f();
        cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfVectorOfDMatch {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfDMatch {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfDMatch {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfDMatch {
    type Item = types::VectorOfDMatch;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfDMatch {
    type Item = types::VectorOfDMatch;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfDMatch>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfDMatch {
    type Storage = types::VectorOfDMatch;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::DMatch>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfDMatch;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", val as "std::vector<cv::DMatch>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        let val = val.as_raw_VectorOfDMatch();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::DMatch>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfDMatch { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfDMatch();
        types::VectorOfDMatch { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::DMatch>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::DMatch>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfDMatch();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfKeyPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfKeyPoint {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfKeyPoint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfKeyPoint {
    type Item = types::VectorOfKeyPoint;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfKeyPoint {
    type Item = types::VectorOfKeyPoint;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfKeyPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfKeyPoint {
    type Storage = types::VectorOfKeyPoint;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::KeyPoint>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfKeyPoint;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", val as "std::vector<cv::KeyPoint>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        let val = val.as_raw_VectorOfKeyPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::KeyPoint>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfKeyPoint { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfKeyPoint();
        types::VectorOfKeyPoint { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::KeyPoint>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfKeyPoint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfMat {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfMat {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfMat(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfMat {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfMat {
    type Item = types::VectorOfMat;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfMat {
    type Item = types::VectorOfMat;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfMat>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfMat {
    type Storage = types::VectorOfMat;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::Mat>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Mat>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Mat>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Mat>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfMat;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", val as "std::vector<cv::Mat>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfMat();
        let val = val.as_raw_VectorOfMat();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", index as "size_t", val as "std::vector<cv::Mat>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfMat();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Mat>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::Mat>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfMat();
        types::VectorOfMat { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::Mat>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::Mat>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfMat();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", index as "size_t", val as "std::vector<cv::Mat>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfMat();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Mat>>*", index as "size_t", val as "std::vector<cv::Mat>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfPoint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfPoint {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfPoint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfPoint {
    type Item = types::VectorOfPoint;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfPoint {
    type Item = types::VectorOfPoint;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint {
    type Storage = types::VectorOfPoint;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::Point>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfPoint;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", val as "std::vector<cv::Point>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfPoint();
        let val = val.as_raw_VectorOfPoint();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::Point>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfPoint { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfPoint();
        types::VectorOfPoint { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::Point>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::Point>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfPoint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfPoint2f {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfPoint2f {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfPoint2f {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfPoint2f {
    type Item = types::VectorOfPoint2f;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfPoint2f {
    type Item = types::VectorOfPoint2f;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint2f>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint2f {
    type Storage = types::VectorOfPoint2f;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::Point2f>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfPoint2f;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", val as "std::vector<cv::Point2f>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        let val = val.as_raw_VectorOfPoint2f();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::Point2f>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfPoint2f { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfPoint2f();
        types::VectorOfPoint2f { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::Point2f>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::Point2f>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfPoint2f();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfRect {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfRect {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfRect {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfRect {
    type Item = types::VectorOfRect;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfRect {
    type Item = types::VectorOfRect;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfRect>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfRect {
    type Storage = types::VectorOfRect;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<cv::Rect>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfRect;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", val as "std::vector<cv::Rect>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfRect();
        let val = val.as_raw_VectorOfRect();
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfRect();
        cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<cv::Rect>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfRect { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfRect();
        types::VectorOfRect { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<cv::Rect>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<cv::Rect>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfRect();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfRect();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfVectorOfint {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfVectorOfint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfVectorOfint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfVectorOfint {
    type Item = types::VectorOfVectorOfint;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfVectorOfint {
    type Item = types::VectorOfVectorOfint;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfVectorOfint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfVectorOfint {
    type Storage = types::VectorOfVectorOfint;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<std::vector<int>>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<std::vector<int>>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<std::vector<int>>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<std::vector<int>>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfVectorOfint;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", val as "std::vector<std::vector<int>>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        let val = val.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", index as "size_t", val as "std::vector<std::vector<int>>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<std::vector<int>>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<std::vector<int>>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfVectorOfint { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfVectorOfint();
        types::VectorOfVectorOfint { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<std::vector<int>>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<std::vector<int>>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", index as "size_t", val as "std::vector<std::vector<int>>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfVectorOfint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<std::vector<int>>>*", index as "size_t", val as "std::vector<std::vector<int>>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfbool {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfbool {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfbool(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfbool {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfbool {
    type Item = types::VectorOfbool;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfbool {
    type Item = types::VectorOfbool;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfbool>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfbool {
    type Storage = types::VectorOfbool;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<bool>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "const std::vector<std::vector<bool>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "const std::vector<std::vector<bool>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "const std::vector<std::vector<bool>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfbool;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", val as "std::vector<bool>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfbool();
        let val = val.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", index as "size_t", val as "std::vector<bool>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfbool();
        cpp!(unsafe [vec as "const std::vector<std::vector<bool>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<bool>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfbool { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfbool();
        types::VectorOfbool { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<bool>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<bool>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfbool();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", index as "size_t", val as "std::vector<bool>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfbool();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<bool>>*", index as "size_t", val as "std::vector<bool>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfchar {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfchar(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfchar {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfchar {
    type Item = types::VectorOfchar;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfchar {
    type Item = types::VectorOfchar;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfchar>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfchar {
    type Storage = types::VectorOfchar;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<char>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfchar;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", val as "std::vector<char>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfchar();
        let val = val.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<char>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<char>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfchar { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfchar();
        types::VectorOfchar { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<char>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<char>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfchar();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfchar();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfint {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfint {
    type Item = types::VectorOfint;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfint {
    type Item = types::VectorOfint;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfint {
    type Storage = types::VectorOfint;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<int>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfint;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfint();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", val as "std::vector<int>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfint();
        let val = val.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfint();
        cpp!(unsafe [vec as "const std::vector<std::vector<int>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<int>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfint();
        types::VectorOfint { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<int>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<int>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfint();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfVectorOfuchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfVectorOfuchar {
    #[inline(always)] pub fn as_raw_VectorOfVectorOfuchar(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfVectorOfuchar {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfVectorOfuchar {
    type Item = types::VectorOfuchar;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfVectorOfuchar {
    type Item = types::VectorOfuchar;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfuchar>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfuchar {
    type Storage = types::VectorOfuchar;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<std::vector<uchar>>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
            vec->clear();
        })
    }

    type Arg = types::VectorOfuchar;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        let val = val.ptr; // fixme use method
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", val as "std::vector<uchar>*"] {
            vec->push_back(*val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfVectorOfuchar();
        let val = val.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] {
            vec->insert(vec->begin() + index, *val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
            try {
                return { Error::Code::StsOk, NULL, new std::vector<uchar>(vec->at(index)) };
            } VEC_CATCH(cv_return_value_void_X)
        }).into_result().map(|ptr| types::VectorOfuchar { ptr })
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfVectorOfuchar();
        types::VectorOfuchar { ptr: cpp!(unsafe [vec_unchkd as "const std::vector<std::vector<uchar>>*", index as "size_t"] -> *mut c_void as "void*" {
            return new std::vector<uchar>((*vec_unchkd)[index]);
        })}
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = *val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfVectorOfuchar();
        let val = val.ptr;
        cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] {
            (*vec)[index] = *val;
        })
    }
}

pub struct VectorOfbool {
    pub(crate) ptr: *mut c_void
}

impl VectorOfbool {
    #[inline(always)] pub fn as_raw_VectorOfbool(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
}

impl Drop for VectorOfbool {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfbool {
    type Item = bool;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfbool {
    type Item = bool;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfbool>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfbool {
    type Storage = bool;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<bool>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "const std::vector<bool>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "const std::vector<bool>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "const std::vector<bool>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*"] {
            vec->clear();
        })
    }

    type Arg = bool;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", val as "bool"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "const std::vector<bool>*", index as "size_t"] -> crate::sys::cv_return_value_bool as "cv_return_value_bool" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_bool)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec_unchkd as "const std::vector<bool>*", index as "size_t"] -> bool as "bool" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfbool();
        cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfchar {
    #[inline(always)] pub fn as_raw_VectorOfchar(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[i8] {
        unsafe {
            let vec_d = self.as_raw_VectorOfchar();
            let data = cpp!(unsafe [vec_d as "std::vector<char>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<char>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfchar {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfchar {
    type Item = i8;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfchar {
    type Item = i8;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfchar>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfchar {
    type Storage = i8;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<char>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "const std::vector<char>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "const std::vector<char>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "const std::vector<char>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*"] {
            vec->clear();
        })
    }

    type Arg = i8;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", val as "char"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "const std::vector<char>*", index as "size_t"] -> crate::sys::cv_return_value_char as "cv_return_value_char" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_char)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec_unchkd as "const std::vector<char>*", index as "size_t"] -> i8 as "char" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfchar();
        cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfdouble {
    pub(crate) ptr: *mut c_void
}

impl VectorOfdouble {
    #[inline(always)] pub fn as_raw_VectorOfdouble(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[f64] {
        unsafe {
            let vec_d = self.as_raw_VectorOfdouble();
            let data = cpp!(unsafe [vec_d as "std::vector<double>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<double>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfdouble {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfdouble {
    type Item = f64;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfdouble {
    type Item = f64;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfdouble>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfdouble {
    type Storage = f64;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<double>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "const std::vector<double>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "const std::vector<double>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "const std::vector<double>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*"] {
            vec->clear();
        })
    }

    type Arg = f64;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", val as "double"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "const std::vector<double>*", index as "size_t"] -> crate::sys::cv_return_value_double as "cv_return_value_double" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_double)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec_unchkd as "const std::vector<double>*", index as "size_t"] -> f64 as "double" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfdouble();
        cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOffloat {
    pub(crate) ptr: *mut c_void
}

impl VectorOffloat {
    #[inline(always)] pub fn as_raw_VectorOffloat(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[f32] {
        unsafe {
            let vec_d = self.as_raw_VectorOffloat();
            let data = cpp!(unsafe [vec_d as "std::vector<float>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<float>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOffloat {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOffloat {
    type Item = f32;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOffloat {
    type Item = f32;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOffloat>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOffloat {
    type Storage = f32;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<float>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "const std::vector<float>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "const std::vector<float>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "const std::vector<float>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*"] {
            vec->clear();
        })
    }

    type Arg = f32;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", val as "float"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "const std::vector<float>*", index as "size_t"] -> crate::sys::cv_return_value_float as "cv_return_value_float" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_float)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec_unchkd as "const std::vector<float>*", index as "size_t"] -> f32 as "float" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOffloat();
        cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfint {
    pub(crate) ptr: *mut c_void
}

impl VectorOfint {
    #[inline(always)] pub fn as_raw_VectorOfint(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[i32] {
        unsafe {
            let vec_d = self.as_raw_VectorOfint();
            let data = cpp!(unsafe [vec_d as "std::vector<int>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<int>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfint {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfint {
    type Item = i32;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfint {
    type Item = i32;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfint>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfint {
    type Storage = i32;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<int>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "const std::vector<int>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "const std::vector<int>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "const std::vector<int>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*"] {
            vec->clear();
        })
    }

    type Arg = i32;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", val as "int"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "const std::vector<int>*", index as "size_t"] -> crate::sys::cv_return_value_int as "cv_return_value_int" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_int)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfint();
        cpp!(unsafe [vec_unchkd as "const std::vector<int>*", index as "size_t"] -> i32 as "int" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfint();
        cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfsize_t {
    pub(crate) ptr: *mut c_void
}

impl VectorOfsize_t {
    #[inline(always)] pub fn as_raw_VectorOfsize_t(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[size_t] {
        unsafe {
            let vec_d = self.as_raw_VectorOfsize_t();
            let data = cpp!(unsafe [vec_d as "std::vector<size_t>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<size_t>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfsize_t {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfsize_t {
    type Item = size_t;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfsize_t {
    type Item = size_t;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfsize_t>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfsize_t {
    type Storage = size_t;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<size_t>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "const std::vector<size_t>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "const std::vector<size_t>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "const std::vector<size_t>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*"] {
            vec->clear();
        })
    }

    type Arg = size_t;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", val as "size_t"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", index as "size_t", val as "size_t"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "const std::vector<std::size_t>*", index as "size_t"] -> crate::sys::cv_return_value_std_size_t as "cv_return_value_std_size_t" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_std_size_t)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec_unchkd as "const std::vector<std::size_t>*", index as "size_t"] -> size_t as "std::size_t" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", index as "size_t", val as "size_t"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfsize_t();
        cpp!(unsafe [vec as "std::vector<size_t>*", index as "size_t", val as "size_t"] {
            (*vec)[index] = val;
        })
    }
}

pub struct VectorOfuchar {
    pub(crate) ptr: *mut c_void
}

impl VectorOfuchar {
    #[inline(always)] pub fn as_raw_VectorOfuchar(&self) -> *mut c_void { self.ptr }

    #[inline]
    pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
        crate::templ::VectorRefIterator::new(self)
    }
    
    fn to_slice(&self) -> &[u8] {
        unsafe {
            let vec_d = self.as_raw_VectorOfuchar();
            let data = cpp!(unsafe [vec_d as "std::vector<uchar>*"] -> *const *mut c_void as "void**" {
                return reinterpret_cast<void**>(vec_d->data());
            });
            let len = cpp!(unsafe [vec_d as "std::vector<uchar>*"] -> size_t as "size_t" {
                return vec_d->size();
            });
            ::std::slice::from_raw_parts(::std::mem::transmute(data), len)
        }
    }
}

impl Drop for VectorOfuchar {
    #[inline]
    fn drop(&mut self) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*"] {
            delete vec;
        })
    }
}

impl IntoIterator for VectorOfuchar {
    type Item = u8;
    type IntoIter = crate::templ::VectorIterator<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'i> IntoIterator for &'i VectorOfuchar {
    type Item = u8;
    type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfuchar>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> crate::templ::Vector<'i> for VectorOfuchar {
    type Storage = u8;

    #[inline]
    fn new() -> Self {
        Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
            return new std::vector<uchar>();
        })}
    }

    #[inline]
    fn len(&self) -> size_t {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<uchar>*"] -> size_t as "size_t" {
            return vec->size();
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<uchar>*"] -> bool as "bool" {
            return vec->empty();
        })
    }

    #[inline]
    fn capacity(&self) -> size_t {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<uchar>*"] -> size_t as "size_t" {
            return vec->capacity();
        })
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*"] {
            vec->shrink_to_fit();
        })
    }                

    #[inline]
    fn reserve(&mut self, additional: size_t) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", additional as "size_t"] {
            vec->reserve(vec->size() + additional);
        })
    }

    #[inline]
    fn remove(&mut self, index: size_t) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t"] {
            vec->erase(vec->begin() + index);
        });
        Ok(())
    }

    #[inline]
    fn clear(&mut self) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*"] {
            vec->clear();
        })
    }

    type Arg = u8;
    
    #[inline]
    fn push(&mut self, val: Self::Arg) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", val as "uchar"] {
            vec->push_back(val);
        })
    }
    
    #[inline]
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] {
            vec->insert(vec->begin() + index, val);
        });
        Ok(())
    }
    
    #[inline]
    fn get(&self, index: size_t) -> crate::Result<Self::Storage> {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "const std::vector<unsigned char>*", index as "size_t"] -> crate::sys::cv_return_value_unsigned_char as "cv_return_value_unsigned_char" {
            try {
                return { Error::Code::StsOk, NULL, vec->at(index) };
            } VEC_CATCH(cv_return_value_unsigned_char)
        }).into_result()
    }
    
    #[inline]
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
        let vec_unchkd = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec_unchkd as "const std::vector<unsigned char>*", index as "size_t"] -> u8 as "unsigned char" {
            return (*vec_unchkd)[index];
        })
    }
    
    #[inline]
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()> {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
            try {
                vec->at(index) = val;
                return { Error::Code::StsOk, NULL };
            } VEC_CATCH(cv_return_value_void)
        }).into_result()
    }
    
    #[inline]
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
        let vec = self.as_raw_VectorOfuchar();
        cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] {
            (*vec)[index] = val;
        })
    }
}

pub use crate::hub::manual::types::*;
