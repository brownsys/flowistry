#![allow(dead_code)]

use std::path::Path;

use anyhow::{anyhow, Context, Result};
use rustc_data_structures::sync::Lrc;
use rustc_hir::{
  intravisit::{self, Visitor},
  itemlikevisit::ItemLikeVisitor,
  BodyId,
};
use rustc_middle::{ty::TyCtxt};
use rustc_span::{FileName, RealFileName, SourceFile, Span};

pub fn qpath_to_span(tcx: TyCtxt, qpath: String) -> Result<Span> {
  struct Finder<'tcx> {
    tcx: TyCtxt<'tcx>,
    qpath: String,
    span: Option<Span>,
  }

  impl Visitor<'tcx> for Finder<'tcx> {
    fn visit_nested_body(&mut self, id: BodyId) {
      intravisit::walk_body(self, self.tcx.hir().body(id));

      let local_def_id = self.tcx.hir().body_owner_def_id(id);
      let function_path = self
        .tcx
        .def_path(local_def_id.to_def_id())
        .to_string_no_crate_verbose();
      if function_path[2 ..] == self.qpath {
        self.span = Some(self.tcx.hir().span(id.hir_id));
      }
    }
  }

  impl ItemLikeVisitor<'hir> for Finder<'tcx>
  where
    'hir: 'tcx,
  {
    fn visit_item(&mut self, item: &'hir rustc_hir::Item<'hir>) {
      <Self as Visitor<'tcx>>::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, impl_item: &'hir rustc_hir::ImplItem<'hir>) {
      <Self as Visitor<'tcx>>::visit_impl_item(self, impl_item);
    }

    fn visit_trait_item(&mut self, _trait_item: &'hir rustc_hir::TraitItem<'hir>) {}
    fn visit_foreign_item(&mut self, _foreign_item: &'hir rustc_hir::ForeignItem<'hir>) {}
  }

  let mut finder = Finder {
    tcx,
    qpath,
    span: None,
  };
  tcx.hir().visit_all_item_likes(&mut finder);
  finder
    .span
    .with_context(|| format!("No function with qpath {}", finder.qpath))
}

pub fn path_to_source_file(
  path: impl AsRef<str>,
  tcx: TyCtxt<'_>,
) -> Result<Lrc<SourceFile>> {
  let source_map = tcx.sess.source_map();
  let files = source_map.files();
  let path = path.as_ref();
  let target_file = Path::new(&path).canonicalize().unwrap();
  files
    .iter()
    .find(|file| {
      if let FileName::Real(RealFileName::LocalPath(other_path)) = &file.name {
        target_file == other_path.canonicalize().unwrap()
      } else {
        false
      }
    })
    .cloned()
    .ok_or_else(|| anyhow!("Could not find file {path} out of files {:#?}", **files))
}