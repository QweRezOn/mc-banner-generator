<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs"
    import { CopyIcon, GemIcon, Import, SlashIcon, SlashSquareIcon } from "lucide-svelte"
    import { Textarea } from "$lib/components/ui/textarea"
    import { Button } from "$lib/components/ui/button"
    import { type BannerPattern, Color, type ColorKey, Pattern, type PatternKey } from "$lib/banner"
    import { getColorString, getPatternString, snakeToPascalCase } from "$lib/utils"
    import type { MinecraftPattern } from "$lib/api"
    import { toast } from "svelte-sonner"

    type Tab = "banner" | "shield"

    interface Props {
        baseColor: Color,
        patterns: BannerPattern[],
    }

    let { baseColor = $bindable(), patterns = $bindable() }: Props = $props()

    let currentTab: Tab = $state("banner")
    let commandValue = $state("meow")

    const onTabChange = (newTab?: string) => {
        currentTab = newTab as Tab
    }

    const onImport = () => {
        // /give @p minecraft:shield[base_color="white",banner_patterns=[{"pattern":"stripe_top","color":"yellow"}]] 1

        const regex = currentTab === "banner"
            ? RegExp("(\\w+)_banner\\[(.*)banner_patterns=\\[(.*)](.*)]")
            : RegExp(`\\[(.*)base_color="(\\w+)"(.*)banner_patterns=\\[(.*)](.*)]`)
        const groupOffset = currentTab === "banner" ? 0 : 1

        const result = regex.exec(commandValue)
        if (!result) {
            toast.error("Invalid command")
            return // todo: regeneraete command maybe?
        }

        const color = Color[snakeToPascalCase(result[1 + groupOffset]) as ColorKey]
        const patternsMinecraft = JSON.parse(`[${result[3 + groupOffset]}]`) as MinecraftPattern[]

        const newPatterns: BannerPattern[] = patternsMinecraft.map(pattern => ({
            id: crypto.randomUUID(),
            pattern: Pattern[snakeToPascalCase(pattern.pattern) as PatternKey],
            color: Color[snakeToPascalCase(pattern.color) as ColorKey],
            hidden: false,
        }))

        baseColor = color
        patterns = newPatterns
        toast.success("Command imported")
    }

    const onCopy = () => {
        navigator.clipboard.writeText(commandValue)
        toast.success("Copied to clipboard")
    }

    const onCopyLink = () => {
        navigator.clipboard.writeText(document.location.href)
        toast.success("Copied to clipboard")
    }

    $effect(() => {
        const minecraftPatterns: MinecraftPattern[] = patterns.map(pattern => ({
            pattern: getPatternString(pattern.pattern),
            color: getColorString(pattern.color),
        }))

        const colorString = getColorString(baseColor)
        const patternsString = JSON.stringify(minecraftPatterns)

        if (currentTab === "banner") {
            commandValue = `/give @p minecraft:${colorString}_banner[banner_patterns=${patternsString}] 1`
        } else {
            commandValue = `/give @p minecraft:shield[base_color="${colorString}",banner_patterns=${patternsString}] 1`
        }
    })
</script>

<Tabs.Root value={currentTab} onValueChange={onTabChange} class="flex flex-col flex-1 gap-2 w-full">
    <Tabs.List class="w-full">
        <Tabs.Trigger value="banner" class="w-full">Banner</Tabs.Trigger>
        <Tabs.Trigger value="shield" class="w-full">Shield</Tabs.Trigger>
    </Tabs.List>

    <div class="flex flex-col gap-2">
        <Textarea bind:value={commandValue} class="min-h-24"></Textarea>
        <Button variant="outline" onclick={onCopy}>
            Copy command
        </Button>
        <Button variant="outline" onclick={onImport}>
            Import command
        </Button>
        <Button variant="outline" onclick={onCopyLink}>
            Copy link
        </Button>
    </div>
</Tabs.Root>
