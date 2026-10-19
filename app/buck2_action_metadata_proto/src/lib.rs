/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

tonic::include_proto!("action_metadata");

pub static REMOTE_DEP_FILE_KEY: &str = "remote_dep_file";

/// Type URL of the `DepFileCandidates` auxiliary metadata: the other stored results of a remote
/// dep-file key, tried when the result returned for the key does not match this build's inputs.
pub static DEP_FILE_CANDIDATES_KEY: &str = "kbuck.DepFileCandidates";
