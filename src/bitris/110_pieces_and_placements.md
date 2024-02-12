# Placements

## Piece

Pieceは、形（Shape）と方向（Orientation）の二つの要素を持ちます。

{{#embed primitives::make_pieces}}

**<span class="caption">コード: Pieceの作成</span>**

### Canonical Piece

`Piece::canonical()`は、Pieceの方向に関係なく、同じフォームを持つPieceを統一された状態で扱うことを可能にします。
これにより、ロジックを簡素化し、一貫性を保つことができます。

{{#embed primitives::piece_canonical}}

## Placements

Placementは、Pieceとその配置する座標を関連付ける概念です。
Placementがどの位置を基準に配置するかは、Placementの表現方法によって異なります。

### 表現方法

#### CcPlacement

CcPlacementは、Pieceの中心軸（[0, 0]）が座標上に来るように配置します。
これは、主にRotation Systemで使用される基準となります。

{{#embed primitives::cc_placement}}

#### BlPlacement

BlPlacementは、ピースを囲う四角形の左下（bottom-left）が座標上に来るように配置します。
これにより、異なるOrientationを持つが同じフォームのピースを統一して扱うことができます。

{{#embed primitives::bl_placement}}

#### TrPlacement

TrPlacementは、ピースを囲う四角形の右上（top-right）が座標上に来るように配置します。
BlPlacementと同様に、異なるOrientationを持つピースを統一して扱うことが可能です。

{{#embed primitives::tr_placement}}

### Placementの相互変換

異なるタイプのPlacementは、自由に相互変換することが可能です。

{{#embed primitives::placement_conversion}}

**<span class="caption">コード: すべて同じ配置を表すPlacement</span>**

### Placementの回転

Placementは、単体で回転させることができます。
この操作では、Placementの表現方法に関わらず、中心軸（cc）を維持しながら回転します。
地形を反映しない回転のため、Kickは発生しません。

{{#embed primitives::rotated_placements}}

## PieceBlocks

PieceBlocksは、ピースのOffsetといくつかの基本情報を持つデータ構造です。
これは、よく参照されるデータのキャッシュオブジェクトであり、事前に計算しておくことで実行時の計算量を減らします。
データのコピーを避け、常に参照（`&`）で保持することが推奨されます。

{{#embed primitives::piece_blocks}}

**<span class="caption">コード: PieceBlocksの取得と基本情報へのアクセス</span>**

{{#embed primitives::piece_blocks_to_locations}}

**<span class="caption">コード: 4つのブロックの座標に変換</span>**
