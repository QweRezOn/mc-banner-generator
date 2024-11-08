import { type BannerPattern, type Color, getRgbColor, Pattern, type PatternKey } from "$lib/banner"
import { apiUrl } from "$lib/api"
import { getPatternString } from "$lib/utils"

const getPixelData = (imageData: ImageData, index: number): [number, number, number, number] => {
    return [imageData.data[index], imageData.data[index + 1], imageData.data[index + 2], imageData.data[index + 3]]
}

const patternImages = new Map<Pattern, HTMLImageElement>()

export const getPatternImage = (pattern: Pattern): Promise<HTMLImageElement> => {
    return new Promise((resolve) => {
        let patternImage = patternImages.get(pattern)

        const loadListener = () => {
            resolve(patternImage!!)
            patternImage!!.removeEventListener("load", loadListener)
        }

        if (!patternImage) {
            patternImage = new Image()
            patternImages.set(pattern, patternImage)
            patternImage.crossOrigin = "anonymous"
            patternImage.src = apiUrl(`/v1/pattern/${getPatternString(pattern)}`)
        }

        if (patternImage.complete) {
            resolve(patternImage)
        } else {
            patternImage.addEventListener("load", loadListener)
        }
    })
}

export const preloadPatterns = () => {
    Object.keys(Pattern)
        .filter(pattern => isNaN(Number(pattern)))
        .map(pattern => Pattern[pattern as PatternKey])
        .forEach(pattern => getPatternImage(pattern))
}

export const renderBanner = async (
    canvas: HTMLCanvasElement,
    baseColor: Color,
    patterns: BannerPattern[],
) => {
    await renderPattern(canvas, Pattern.Base, baseColor, true)
    for (const pattern of patterns.filter(pattern => !pattern.hidden)) {
        await renderPattern(canvas, pattern.pattern, pattern.color)
    }
}

export const renderPattern = async (
    canvas: HTMLCanvasElement,
    pattern: Pattern,
    patternColor: Color,
    clear = false,
) => {
    const patternImage = await getPatternImage(pattern)
    const color = getRgbColor(patternColor)

    const context = canvas.getContext("2d", { willReadFrequently: true })!!
    if (clear) {
        context.clearRect(0, 0, canvas.width, canvas.height)
    }
    const canvasData = context.getImageData(0, 0, canvas.width, canvas.height)
    context.clearRect(0, 0, canvas.width, canvas.height)

    context.drawImage(patternImage, 0, 0)
    const patternData = context.getImageData(0, 0, canvas.width, canvas.height)

    for (let index = 0; index < canvasData.data.length; index += 4) {
        const patternPixel = getPixelData(patternData, index)
        if (patternPixel[3] === 0) {
            continue
        }

        const pixel = getPixelData(canvasData, index)

        const brightnessR = patternPixel[0] / 255.0
        const brightnessG = patternPixel[1] / 255.0
        const brightnessB = patternPixel[2] / 255.0

        if (pixel[3] === 0) {
            canvasData.data[index] = color[0] * brightnessR
            canvasData.data[index + 1] = color[1] * brightnessG
            canvasData.data[index + 2] = color[2] * brightnessB
            canvasData.data[index + 3] = patternPixel[3]
        } else {
            const alpha = patternPixel[3] / 255.0

            canvasData.data[index] = (pixel[0] * (1.0 - alpha) + color[0] * alpha) * brightnessR
            canvasData.data[index + 1] = (pixel[1] * (1.0 - alpha) + color[1] * alpha) * brightnessG
            canvasData.data[index + 2] = (pixel[2] * (1.0 - alpha) + color[2] * alpha) * brightnessB
            canvasData.data[index + 3] = 255
        }
    }

    context.putImageData(canvasData, 0, 0)
}
