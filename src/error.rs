//! Esse módulo implementa ferramentas básicas para gerenciar erros de parse/compilação.

#[cfg(test)]
mod test;

use crate::position::Span;
use std::{error::Error as StdError, fmt, slice, vec};

/// Um erro lançado pelo lexer/parser/compilador.
#[derive(Debug)]
pub struct Error {
    /// Causa do erro: detalhes em específico do erro.
    cause: Box<dyn StdError + Send + Sync>,
    /// 'Janela' (span) localizada no código fonte, apontado para a causa
    /// do erro no código fonte. Pode não estar presente.
    span: Option<Span>,
}

impl Error {
    /// Cria um erro a partir da causa de do span apontando para onde o erro ocorre
    /// no código fonte.
    pub fn new<E>(cause: E, span: Span) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self { cause: Box::new(cause), span: Some(span) }
    }

    /// Cria um erro a partir da causa mas sem um local específico no código fonte.
    pub fn with_no_span<E>(cause: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self { cause: Box::new(cause), span: None }
    }

    /// O span (janela) da localização no código onde esse erro ocorreu.
    /// Pode não haver tal localização.
    #[inline]
    pub fn span(&self) -> Option<Span> {
        self.span
    }

    /// A causa do erro, i.e. os detalhes específicos do erro.
    #[inline]
    pub fn cause(&self) -> &(dyn StdError + Send + Sync + 'static) {
        &*self.cause
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.cause)?;
        if let Some(span) = self.span() {
            write!(formatter, ", {}", span)?;
        }
        Ok(())
    }
}

impl StdError for Error {}

/// Uma lista de diagnósticos, i.e. lista de erros de lexing/parsing/compiling.
#[derive(Debug)]
pub struct Diagnostics {
    /// Um vetor de erros implementando a tal lista.
    errors: Vec<Error>,
}

impl Default for Diagnostics {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Diagnostics {
    /// Cria uma lista de diagnósticos vazia.
    #[inline]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Aciona um erro e salva-o na lista de diagnósticos.
    #[inline]
    pub fn raise(&mut self, error: Error) {
        self.errors.push(error);
    }

    /// O resultado da compilação está OK? É um sucesso?
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// O resultado da compilação é erro? É uma falha?
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Conta quantos erros foram acionados nessa lista de diagnósticos.
    #[inline]
    pub fn count_errors(&self) -> usize {
        self.errors.len()
    }

    /// Itera pela lista de diagnósticos por referência.
    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
    }
}

impl IntoIterator for Diagnostics {
    type Item = Error;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self.errors.into_iter() }
    }
}

impl<'diag> IntoIterator for &'diag Diagnostics {
    type Item = &'diag Error;
    type IntoIter = Iter<'diag>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter { inner: self.errors.iter() }
    }
}

/// Iterador por valor (owned) sobre a lista de diagnósticos/erros.
#[derive(Debug)]
pub struct IntoIter {
    /// O tipo por baixo dos panos que realmente itera.
    inner: vec::IntoIter<Error>,
}

impl Iterator for IntoIter {
    type Item = Error;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl ExactSizeIterator for IntoIter {}

/// Iterador por referência (borrowed) sobre a lista de diagnósticos/erros.
#[derive(Debug)]
pub struct Iter<'diagnostics> {
    /// O tipo por baixo dos panos que realmente itera.
    inner: slice::Iter<'diagnostics, Error>,
}

impl<'diagnostics> Iterator for Iter<'diagnostics> {
    type Item = &'diagnostics Error;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'diagnostics> DoubleEndedIterator for Iter<'diagnostics> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'diagnostics> ExactSizeIterator for Iter<'diagnostics> {}
