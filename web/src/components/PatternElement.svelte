<script lang="ts">
    import * as Popover from "$lib/components/ui/popover"
    import { Color, type ColorKey, getHexColor, Pattern, type PatternKey } from "$lib/banner"
    import { EyeClosedIcon, EyeIcon, GripVerticalIcon, TrashIcon } from "lucide-svelte"
    import { cn } from "$lib/utils"
    import PatternCanvas from "./PatternCanvas.svelte"
    import { Button } from "$lib/components/ui/button"
    import type { SvelteHTMLElements } from "svelte/elements"
    import { dragHandle } from "svelte-dnd-action"

    type Props = {
        patternColor: Color,
        pattern: Pattern,
        patternHidden?: boolean,
        disabled?: boolean,
        // eslint-disable-next-line no-unused-vars
        onVisibilityChange?: (hidden: boolean) => void,
        onRemove?: () => void,
    } & SvelteHTMLElements["div"]

    type $$Props = Props

    let {
        disabled,
        patternColor = $bindable(),
        pattern = $bindable(),
        patternHidden,
        onVisibilityChange,
        onRemove,
        ...rest
    }: Props = $props()

    let oldColor: Color | undefined = $state()
    let oldPattern: Pattern | undefined = $state()

    let colorOpen = $state(false)

    let patternOpen = $state(false)

    const patterns = Object.keys(Pattern)
        .filter(key => isNaN(Number(key)))
        .map(key => Pattern[key as PatternKey])
        .filter(key => key !== Pattern.Base)

    const colors = Object.keys(Color)
        .filter(key => isNaN(Number(key)))
        .map(key => key as ColorKey)

    const setColor = (newColor: Color) => {
        patternColor = newColor
        oldColor = undefined
        colorOpen = false
    }

    const setPattern = (newPattern: Pattern) => {
        pattern = newPattern
        oldPattern = undefined
        patternOpen = false
    }

    const onPatternOpenChange = (state: boolean) => {
        patternOpen = state

        if (oldPattern) {
            pattern = oldPattern
            oldPattern = undefined
        }
    }

    const onColorOpenChange = (state: boolean) => {
        colorOpen = state

        if (oldColor !== undefined) {
            patternColor = oldColor
            oldColor = undefined
        }
    }
</script>

<div
    class={cn("flex gap-2 items-center w-full p-4 bg-background border border-accent rounded-lg", disabled && "pl-12")}
    {...rest}
>
    {#if !disabled}
        <div use:dragHandle class="outline-none">
            <GripVerticalIcon class="text-accent-foreground" />
        </div>
    {/if}

    <Popover.Root open={patternOpen} onOpenChange={onPatternOpenChange}>
        {#if disabled}
            <PatternCanvas pattern={pattern} patternColor={patternColor} />
        {:else}
            <Popover.Trigger>
                <PatternCanvas pattern={pattern} patternColor={patternColor} />
            </Popover.Trigger>
        {/if}
        <Popover.Content class="flex flex-wrap gap-2">
            {#each patterns as patternKey}
                <button
                    onclick={() => setPattern(patternKey)}
                    onmouseenter={() => {
                        if (!oldPattern) oldPattern = pattern
                        pattern = patternKey
                    }}
                    aria-label="Pattern {patternKey}"
                >
                    <PatternCanvas
                        pattern={patternKey}
                        patternColor={patternColor}
                        class="w-5 h-auto hover:outline hover:outline-zinc-500 bg-accent"
                    />
                </button>
            {/each}
        </Popover.Content>
    </Popover.Root>

    <Popover.Root open={colorOpen} onOpenChange={onColorOpenChange}>
        <Popover.Trigger>
            <div
                style="background-color: #{getHexColor(patternColor).toString(16)}"
                class="size-4 rounded"
            ></div>
        </Popover.Trigger>
        <Popover.Content class="flex flex-wrap gap-2 w-[218px]">
            {#each colors as colorKey}
                <button
                    style="background-color: #{getHexColor(Color[colorKey]).toString(16)}"
                    class="size-4 rounded cursor-pointer hover:outline hover:outline-zinc-500"
                    onclick={() => setColor(Color[colorKey])}
                    onmouseenter={() => {
                        if (oldColor === undefined) oldColor = patternColor
                        patternColor = Color[colorKey]
                    }}
                    aria-label="Color {colorKey}"
                ></button>
            {/each}
        </Popover.Content>
    </Popover.Root>

    <div class="flex-1"></div>

    {#if !disabled}
        <Button variant="ghost" size="icon" onclick={() => onVisibilityChange?.(!patternHidden)} >
            {#if patternHidden}
                <EyeClosedIcon/>
            {:else}
                <EyeIcon/>
            {/if}
        </Button>

        <Button variant="ghost" size="icon" onclick={onRemove} >
            <TrashIcon />
        </Button>
    {/if}
</div>
