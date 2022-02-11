var searchIndex = JSON.parse('{\
"pinguim_language":{"doc":"Essa crate fornece algumas ferramentas para …","t":[0,0,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["error","position","Diagnostics","Error","IntoIter","Iter","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cause","count_errors","default","fmt","fmt","fmt","fmt","fmt","from","from","from","from","into","into","into","into","into_iter","into_iter","into_iter","is_err","is_ok","iter","new","new","next","next","next_back","next_back","raise","size_hint","size_hint","span","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","with_no_span","Position","Span","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","column","default","default","end","eq","eq","finish","fmt","fmt","fmt","fmt","from","from","from_start","hash","hash","into","into","line","ne","ne","partial_cmp","partial_cmp","start","to_owned","to_owned","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","update","update","utf16_index","utf8_index"],"q":["pinguim_language","","pinguim_language::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","pinguim_language::position","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Esse módulo implementa ferramentas básicas para …","Esse módulo define itens para rastrear posição e …","Uma lista de diagnósticos, i.e. lista de erros de …","Um erro lançado pelo lexer/parser/compilador.","Iterador por valor (owned) sobre a lista de …","Iterador por referência (borrowed) sobre a lista de …","","","","","","","","","A causa do erro, i.e. os detalhes específicos do erro.","Conta quantos erros foram acionados nessa lista de …","","","","","","","","","","","","","","","","","","O resultado da compilação é erro? É uma falha?","O resultado da compilação está OK? É um sucesso?","Itera pela lista de diagnósticos por referência.","Cria um erro a partir da causa de do span apontando para …","Cria uma lista de diagnósticos vazia.","","","","","Aciona um erro e salva-o na lista de diagnósticos.","","","O span (janela) da localização no código onde esse erro …","","","","","","","","","","","","","","Cria um erro a partir da causa mas sem um local …","Uma posição “pontual” no código fonte.","Uma ‘janela’ (span) localizada no código fonte. …","","","","","","","","","","","Coluna (a partir do 1) do caractere","","","Posição final do span (exclusiva)","","","Finaliza o Span, de forma que o início é avançado para …","","","","","","","Cria uma estrutura Span com começo e fim no mesmo lugar","","","","","Linha (a partir do 1) do caractere","","","","","Posição inicial do span (inclusiva)","","","","","","","","","","","Atualiza nova linha e nova coluna de acordo com o …","Atualiza posição final do símbolo","Índice (a partir do 0) do caractere na string em UTF-16 …","Índice (a partir do 0) do caractere na string em UTF-8 …"],"i":[0,0,0,0,0,0,1,2,3,4,1,2,3,4,1,2,2,1,1,2,3,4,1,2,3,4,1,2,3,4,2,3,4,2,2,2,1,2,3,4,3,4,2,3,4,1,1,1,2,3,4,1,2,3,4,1,2,3,4,1,0,0,5,6,5,6,5,6,5,6,5,6,5,5,6,6,5,6,6,5,5,6,6,5,6,6,5,6,5,6,5,5,6,5,6,6,5,6,5,6,5,6,5,6,5,6,5,6,5,5],"f":[null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["stderror",8]],[[],["usize",15]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["iter",3]],[[["span",3]]],[[]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[["error",3]]],[[]],[[]],[[],["option",4,[["span",3]]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],null,null,[[]],[[]],[[]],[[]],[[],["position",3]],[[],["span",3]],[[]],[[]],[[["position",3]],["ordering",4]],[[["span",3]],["ordering",4]],null,[[]],[[]],null,[[["position",3]],["bool",15]],[[["span",3]],["bool",15]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["position",3]]],[[]],[[]],[[]],[[]],null,[[["position",3]],["bool",15]],[[["span",3]],["bool",15]],[[["position",3]],["option",4,[["ordering",4]]]],[[["span",3]],["option",4,[["ordering",4]]]],null,[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[["char",15]]],[[["char",15]]],null,null],"p":[[3,"Error"],[3,"Diagnostics"],[3,"IntoIter"],[3,"Iter"],[3,"Position"],[3,"Span"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};