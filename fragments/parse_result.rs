// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

/// The standard result for any grammars's parse attempt.
///
/// Success: Returns the updated Context and the resulting CST node.
/// Failure: Returns the Context (to preserve pos and cut_seen) and an error message.
pub type ParseResult<C> = Result<(Ctx<C>, Cst), (Ctx<C>, String)>;
