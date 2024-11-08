<script lang="ts">

    import PatternElement from "./components/PatternElement.svelte"
    import { type BannerPattern, Color, type ColorKey, Pattern, type PatternKey } from "$lib/banner"
    import { Button } from "$lib/components/ui/button"
    import { PlusIcon } from "lucide-svelte"
    import { type DndEvent, dndzone } from "svelte-dnd-action"
    import { onMount } from "svelte"
    import { snakeToPascalCase } from "$lib/utils"
    import { getBannerMeta } from "$lib/api"
    import Actions from "./components/Actions.svelte"
    import { Toaster } from "$lib/components/ui/sonner"
    import BannerCanvas from "./components/BannerCanvas.svelte"
    import { preloadPatterns } from "$lib/banner-render"

    let baseColor = $state(Color.White)
    let patterns: BannerPattern[] = $state([])
    let loading = $state(true)

    let timeout: ReturnType<typeof setTimeout> | undefined

    const createPattern = (pattern: Pattern) => {
        patterns.push({
            id: crypto.randomUUID(),
            pattern,
            hidden: false,
            color: Color.White,
        })
    }

    const removePattern = (id: string) => {
        patterns = patterns.filter(pattern => pattern.id !== id)
    }

    const handleSort = (e: CustomEvent<DndEvent>) => {
        patterns = e.detail.items as BannerPattern[]
    }

    const clear = () => {
        baseColor = Color.White
        patterns = []
    }

    const url = $derived.by(() => {
        const colorHex = baseColor.toString(16)

        const patternsHex = patterns
            .filter(pattern => !pattern.hidden)
            .map((pattern) => {
                let patternHex = pattern.pattern.toString(16)
                if (patternHex.length === 1) {
                    patternHex = "0" + patternHex
                }
                const colorHex = pattern.color.toString(16)

                return patternHex + colorHex
            })
            .join("")

        return colorHex + patternsHex
    })

    $effect(() => {
        if (loading) return

        const changedUrl = url

        if (timeout) clearTimeout(timeout)
        timeout = setTimeout(() => {
            const searchParams = new URLSearchParams(document.location.search)
            if ((baseColor === Color.White && !patterns.length) && !searchParams.has("banner")) return
            if (changedUrl === searchParams.get("banner")) return

            searchParams.set("banner", changedUrl)

            const newUrl = [
                window.location.protocol,
                "//",
                window.location.host,
                window.location.pathname,
                "?",
                searchParams.toString(),
            ].join("")
            window.history.pushState({ path: newUrl }, "", newUrl)

            // todo: not really sure about this, but it will allow reading url from parent window
            const windowOrigin = searchParams.get("origin")
            if (windowOrigin) {
                window.parent.postMessage({ changedUrl }, windowOrigin)
            }
        }, 1000)
    })

    onMount(async () => {
        preloadPatterns()

        const searchParams = new URLSearchParams(document.location.search)
        const banner = searchParams.get("banner")
        if (!banner) {
            loading = false
            return
        }

        // todo: it's possible to do it on client side now
        const meta = await getBannerMeta(banner)

        baseColor = Color[snakeToPascalCase(meta.color) as ColorKey]

        patterns = meta.patterns.map(pattern => ({
            id: crypto.randomUUID(),
            pattern: Pattern[snakeToPascalCase(pattern.pattern) as PatternKey],
            color: Color[snakeToPascalCase(pattern.color) as ColorKey],
        } as BannerPattern))

        loading = false
    })
</script>

<Toaster position="top-center" />

<main class="flex flex-col gap-8 md:container rendering-pixelated py-8 px-8 xl:px-64 md:py-16 max-md:text-center">
    <h1 class="text-4xl text-accent-foreground font-semibold">Banner Generator</h1>

    {#if !loading}
        <div class="flex max-md:flex-col max-md:items-center gap-8">
            <div class="flex flex-col gap-4 w-32">
                <BannerCanvas baseColor={baseColor} patterns={patterns} />
                <Button variant="destructive" class="w-full" onclick={clear}>Clear</Button>
            </div>
            <div class="flex flex-col gap-4 w-64 max-md:w-full">
                <PatternElement pattern={Pattern.Base} bind:color={baseColor} disabled />

                {#if patterns.length > 0}
                    <div
                        class="flex flex-col w-full gap-4 !outline-none"
                        use:dndzone="{{ items: patterns }}"
                        onconsider="{handleSort}"
                        onfinalize="{handleSort}"
                    >
                        {#each patterns as pattern(pattern.id)}
                            <PatternElement
                                bind:pattern={pattern.pattern}
                                bind:color={pattern.color}
                                patternHidden={pattern.hidden}
                                onVisibilityChange={hidden => pattern.hidden = hidden}
                                onRemove={() => removePattern(pattern.id)}
                            />
                        {/each}
                    </div>
                {/if}

                {#if patterns.length < 16}
                    <Button variant="outline" onclick={() => createPattern(Pattern.Bricks)} >
                        <PlusIcon />
                    </Button>
                {/if}
            </div>

            <Actions bind:baseColor={baseColor} bind:patterns={patterns} />
        </div>
    {/if}
</main>
