//! Automation the [KISS](https://en.wikipedia.org/wiki/KISS_principle) way.
//!
//! # Introduction
//!
//! [Ansible](https://ansible.com) is a great tool for automation. But it
//! suffers from the same problem of many such tools: a big pile of custom YAML
//! DSL.
//!
//! YAML is used to provide a declarative syntax of your automated workflow.
//! This is nice for simple use cases, but automation can become rather complex
//! very quickly.
//!
//! Then those tools implement control flow structures (conditional execution,
//! loops, parallelization, ...), then the ability to save values into
//! variables.
//!
//! Before you know it, you're programming in YAML. And the developer experience
//! of such a language is terrible.
//!
//! **tricorder** aims to fix this. It gives you a single tool to perform tasks
//! on multiple remotes. You then use your common UNIX tools like `bash`, `jq`,
//! `curl`, etc... to compose those tasks together.
//!
//! # About tricorder
//!
//! The name comes from
//! [Star Trek's Tricorder](https://en.wikipedia.org/wiki/Tricorder), a
//! multifunction hand-held device to perform sensor environment scans, data
//! recording, and data analysis. Pretty much anything required by the plot.
//!
//! The main goal of **tricorder** is to provide the basic tools to perform
//! tasks on remote hosts and get out of your way. Allowing you to integrate it
//! with any scripting language or programming language of your choice, instead
//! of forcing you to develop in a sub-par custom YAML DSL.

pub mod inventory;
pub mod host;
pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use self::{
  inventory::Inventory,
  host::Host,
};
