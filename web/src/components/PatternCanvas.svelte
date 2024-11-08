<script lang="ts">
    import { Color, type Pattern } from "$lib/banner"
    import { renderPattern } from "$lib/banner-render"
    import { cn } from "$lib/utils"

    interface Props {
        class?: string,
        pattern: Pattern,
        patternColor: Color,
    }
    let { pattern, patternColor, ...props }: Props = $props()

    let canvas: HTMLCanvasElement | undefined = $state()

    $effect(() => {
        if (!canvas) return
        renderPattern(canvas, pattern, patternColor, true)
    })
</script>

<canvas
    bind:this={canvas}
    width={20}
    height={40}
    class={cn("rendering-pixelated w-8 h-16 bg-accent", props.class)}
></canvas>
