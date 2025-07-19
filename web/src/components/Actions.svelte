<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs"
    import { CopyIcon, GemIcon, Import, SlashIcon, SlashSquareIcon } from "lucide-svelte"
    import { CopyIcon, GemIcon, Import, SlashIcon, SlashSquareIcon, RefreshCw } from "lucide-svelte"
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

    
    const regenerateCommand = () => {
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
        
        toast.success("Команда регенерирована")
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
            return
            toast.error("Неверная команда. Регенерирую команду из текущих настроек...")
            regenerateCommand()
            return
        }
        
        try {
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
            toast.success("Команда импортирована")
        } catch (error) {
            toast.error("Ошибка при импорте. Регенерирую команду...")
            regenerateCommand()
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
        toast.success("Команда импортирована")
    }

    
    const onCopy = () => {
        navigator.clipboard.writeText(commandValue)
        toast.success("Скопировано в буфер обмена")
    }

    
    const onCopyLink = () => {
        navigator.clipboard.writeText(document.location.href)
        toast.success("Скопировано в буфер обмена")
    }

    
    const onClear = () => {
        patterns = []
        toast.success("Все паттерны удалены")
    }
    
    const onRandomize = () => {
        const colors = Object.values(Color)
        const patternTypes = Object.values(Pattern)
        
        baseColor = colors[Math.floor(Math.random() * colors.length)]
        
        const numPatterns = Math.floor(Math.random() * 6) + 1
        const newPatterns: BannerPattern[] = []
        
        for (let i = 0; i < numPatterns; i++) {
            newPatterns.push({
                id: crypto.randomUUID(),
                pattern: patternTypes[Math.floor(Math.random() * patternTypes.length)],
                color: colors[Math.floor(Math.random() * colors.length)],
                hidden: false,
            })
        }
        
        patterns = newPatterns
        toast.success("Случайный дизайн создан!")
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
@@ -85,17 +138,37 @@
        <Tabs.Trigger value="banner" class="w-full">Баннер</Tabs.Trigger>
        <Tabs.Trigger value="shield" class="w-full">Щит</Tabs.Trigger>
    </Tabs.List>

    
    <div class="flex flex-col gap-2">
        <Textarea bind:value={commandValue} class="min-h-24"></Textarea>
        <Button variant="outline" onclick={onCopy}>
            Скопировать команду
        </Button>
        <Button variant="outline" onclick={onImport}>
            Импортировать команду
        </Button>
        <Button variant="outline" onclick={onCopyLink}>
            Скопировать ссылку
        </Button>
        
        <div class="grid grid-cols-2 gap-2">
            <Button variant="outline" onclick={onCopy}>
                <CopyIcon class="w-4 h-4 mr-2" />
                Скопировать команду
            </Button>
            <Button variant="outline" onclick={onImport}>
                <Import class="w-4 h-4 mr-2" />
                Импортировать команду
            </Button>
            <Button variant="outline" onclick={onCopyLink}>
                <SlashIcon class="w-4 h-4 mr-2" />
                Скопировать ссылку
            </Button>
            <Button variant="outline" onclick={regenerateCommand}>
                <RefreshCw class="w-4 h-4 mr-2" />
                Регенерировать
            </Button>
        </div>
        
        <div class="grid grid-cols-2 gap-2">
            <Button variant="destructive" onclick={onClear}>
                Очистить все
            </Button>
            <Button variant="secondary" onclick={onRandomize}>
                <GemIcon class="w-4 h-4 mr-2" />
                Случайный дизайн
            </Button>
        </div>
    </div>
</Tabs.Root>
</Tabs.Root>