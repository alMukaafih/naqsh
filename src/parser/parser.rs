#![allow(dead_code, unused_variables)]
use crate::image::Image;
use super::Manifest;

pub struct Parser {
    manifest: Manifest
}

impl Parser {
    /// Creates a new Parser
    pub fn new(manifest: Manifest) -> Self {
        Self { manifest }
    }

    /// Parses the Manifest file and return equivalent [Image].
    pub fn parse(&self) -> Image {
        for object in &self.manifest.objects {

        }
        todo!()
    }
}