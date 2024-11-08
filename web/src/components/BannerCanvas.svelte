<script lang="ts">
    import { type BannerPattern, Color } from "$lib/banner"
    import { renderBanner } from "$lib/banner-render"

    interface Props {
        baseColor: Color,
        patterns: BannerPattern[],
    }
    let { baseColor, patterns }: Props = $props()

    let canvas: HTMLCanvasElement | undefined = $state()

    $effect(() => {
        if (!canvas) return
        JSON.stringify(patterns) // hack to track changes
        renderBanner(canvas, baseColor, patterns)
    })

    $inspect(patterns)
</script>

<canvas
    bind:this={canvas}
    width={20}
    height={40}
    class="rendering-pixelated w-full h-64"
></canvas>
