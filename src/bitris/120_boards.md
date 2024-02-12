# Boards

## Make

Boardは、ブロックの有無を持っているstructです。
Boardの名前の後ろにある数字は、ブロックを持つ変数のビット幅を表し、これがボードの高さに相当します。

{{#embed boards::make_boards}}

**<span class="caption">コード: 空のBoardを作成</span>**

{{#embed boards::make_board_from_str}}

**<span class="caption">コード: 文字列からBoardを作成</span>**

## Operation

Boardからブロックを操作することができます。
これらの操作は破壊的な操作であることに注意してください。

{{#embed boards::set_and_unset}}

**<span class="caption">コード: Set and Unset</span>**

### Place On

Placementを使って、Boardに対してPieceの配置を指示できます。
`Placement::place_on()`はライン消去を行いません。
ライン消去も同時に行う場合は、`Placement::place_on_and_clear_lines()`を利用してください。

{{#embed boards::place_on}}

**<span class="caption">コード: `Board::place_on()`</span>**

{{#embed boards::place_on_and_clear_lines}}

**<span class="caption">コード: `Board::place_on_and_clear_lines()`</span>**

## Converting

異なるビット幅のBoardは、自由に相互変換することが可能です。
Boardに収まらないブロックは切り捨てられます。

{{#embed boards::convert}}
