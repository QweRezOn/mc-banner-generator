/* eslint-disable no-unused-vars */

export interface BannerPattern {
    id: string,
    pattern: Pattern,
    hidden: boolean,
    color: Color,
}

export enum Pattern {
    Base,
    Border,
    Bricks,
    Circle,
    Creeper,
    Cross,
    CurlyBorder,
    DiagonalLeft,
    DiagonalRight,
    DiagonalUpLeft,
    DiagonalUpRight,
    Flow,
    Flower,
    Globe,
    Gradient,
    GradientUp,
    Guster,
    HalfHorizontal,
    HalfHorizontalBottom,
    HalfVertical,
    HalfVerticalRight,
    Mojang,
    Piglin,
    Rhombus,
    Skull,
    SmallStripes,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    StraightCross,
    StripeBottom,
    StripeCenter,
    StripeDownleft,
    StripeDownright,
    StripeLeft,
    StripeMiddle,
    StripeRight,
    StripeTop,
    TrianglesBottom,
    TrianglesTop,
    TriangleBottom,
    TriangleTop,
}

export type PatternKey = keyof typeof Pattern

export enum Color {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

export type ColorKey = keyof typeof Color

export const getHexColor = (color: Color): number => {
    switch (color) {
        case Color.White: return 0xF9FFFE
        case Color.Orange: return 0xF9801D
        case Color.Magenta: return 0xC74EBD
        case Color.LightBlue: return 0x3AB3DA
        case Color.Yellow: return 0xFED83D
        case Color.Lime: return 0x80C71F
        case Color.Pink: return 0xF38BAA
        case Color.Gray: return 0x474F52
        case Color.LightGray: return 0x9D9D97
        case Color.Cyan: return 0x169C9C
        case Color.Purple: return 0x8932B8
        case Color.Blue: return 0x3C44AA
        case Color.Brown: return 0x835432
        case Color.Green: return 0x5E7C16
        case Color.Red: return 0xB02E26
        case Color.Black: return 0x1D1D21
    }
}

export type RgbColor = [number, number, number]

export const getRgbColor = (color: Color): RgbColor => {
    const hex = getHexColor(color)

    const r = (hex >> 16) & 0xFF
    const g = (hex >> 8) & 0xFF
    const b = hex & 0xFF

    return [r, g, b]
}
