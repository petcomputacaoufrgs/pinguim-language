//! Esse módulo define itens para rastrear posição e localização em um código fonte.

#[cfg(test)]
mod test;

use std::fmt;

/// Uma posição "pontual" no código fonte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// Índice (a partir do 0) do caractere na string em UTF-8 (para localização
    /// absoluta no Rust)
    pub utf8_index: usize,
    /// Índice (a partir do 0) do caractere na string em UTF-16 (para localização
    /// absoluta no JavaScript)
    pub utf16_index: usize,
    /// Linha (a partir do 1) do caractere
    pub line: u64,
    /// Coluna (a partir do 1) do caractere
    pub column: u64,
}

/// Implementa trait Default para criar uma estrutura Position com os atributos
/// como se o caractere estivesse na primeira posição
impl Default for Position {
    fn default() -> Self {
        Position { utf8_index: 0, utf16_index: 0, line: 1, column: 1 }
    }
}

impl Position {
    /// Atualiza nova linha
    #[inline]
    fn update_newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Atualiza nova coluna
    #[inline]
    fn update_column(&mut self) {
        self.column += 1;
    }

    /// Atualiza índices conforme caractere
    ///
    /// - `character`: novo caractere lido
    #[inline]
    fn update_indices(&mut self, character: char) {
        self.utf8_index += character.len_utf8();
        self.utf16_index += character.len_utf16();
    }

    /// Atualiza nova linha e nova coluna de acordo com o caractere lido
    ///
    /// - `character`: novo caractere lido
    #[inline]
    pub fn update(&mut self, character: char) {
        self.update_indices(character);
        if character == '\n' {
            self.update_newline();
        } else {
            self.update_column()
        }
    }
}

/// Implementa trait display para a parte de posição em mensagens de erro
impl fmt::Display for Position {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "linha {} e coluna {}", self.line, self.column)
    }
}

/// Uma 'janela' (span) localizada no código fonte. Diferente de [`Position`],
/// o Span não é um único ponto, mas tem um "tamanho".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    /// Posição inicial do span (inclusiva)
    pub start: Position,
    /// Posição final do span (exclusiva)
    pub end: Position,
}

/// Implementa trait Default para criar um Span com começo e fim no mesmo lugar
impl Default for Span {
    #[inline]
    fn default() -> Self {
        Self::from_start(Position::default())
    }
}

impl Span {
    /// Cria uma estrutura Span com começo e fim no mesmo lugar
    ///
    /// - `start`: posição inicial do caractere
    #[inline]
    pub fn from_start(start: Position) -> Self {
        Self { start, end: start }
    }

    /// Atualiza posição final do símbolo
    ///
    /// - `character`:  novo caractere lido
    #[inline]
    pub fn update(&mut self, character: char) {
        self.end.update(character);
    }

    /// Finaliza o Span, de forma que o início é avançado para a mesma posição
    /// do final, e portanto, passa a ter "tamanho zero", mas ao mesmo tempo,
    /// continua de onde havia parado.
    #[inline]
    pub fn finish(&mut self) {
        self.start = self.end;
    }
}

// Implementa trait display para a parte de span em mensagens de erro, convertendo
// a posição final "exclusiva" para "inclusiva".
impl fmt::Display for Span {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let end = Position { column: self.end.column - 1, ..self.end };
        if self.start.line != self.end.line {
            write!(formatter, "da {}, até a {}", self.start, end)
        } else if self.start.column + 1 == self.end.column {
            write!(formatter, "na {}", self.start)
        } else {
            write!(formatter, "da {}, até a coluna {}", self.start, end.column)
        }
    }
}
