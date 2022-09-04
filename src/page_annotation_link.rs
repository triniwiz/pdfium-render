//! Defines the [PdfPageLinkAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Link`.

use std::os::raw::c_int;
use crate::action::PdfAction;
use crate::bindgen::FPDF_ANNOTATION;
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::document::PdfDocument;

pub struct PdfPageLinkAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    document: &'a PdfDocument<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageLinkAnnotation<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_ANNOTATION,
        document: &'a PdfDocument<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageLinkAnnotation { handle, document, bindings }
    }

    pub fn action(&self) -> Option<PdfAction<'a>> {
        let link = self.bindings.FPDFAnnot_GetLink(self.handle);
        let handle = self.bindings.FPDFLink_GetAction(link);

        if handle.is_null() {
            None
        } else {
            Some(PdfAction::from_pdfium(handle, self.document, self.bindings))
        }
    }

    pub fn uri_path(&self) -> Option<String> {
        if let Some(action) = self.action() {
            let document = *self.document.get_handle();
            let size = self.bindings
                .FPDFAction_GetURIPath(document, *action.get_handle(), std::ptr::null_mut(), 0);

            if size == 0 {
                return None;
            }

            let mut buf = String::with_capacity(size as usize);

            unsafe {
                self.bindings
                    .FPDFAction_GetURIPath(document, *action.get_handle(), buf.as_mut_vec().as_mut_ptr().cast(), size);
            }

            return Some(buf);
        }
        None
    }

    pub fn dest(&self) -> Option<c_int> {
        if let Some(action) = self.action() {
            let document_handle = *self.document.get_handle();
            let dest = self.bindings.FPDFAction_GetDest(document_handle, *action.get_handle());

            if !dest.is_null() {
                return Some(self.bindings.FPDFDest_GetDestPageIndex(document_handle, dest))
            }
        }
        None
    }
}

impl<'a> PdfPageAnnotationPrivate for PdfPageLinkAnnotation<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_ANNOTATION {
        &self.handle
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
