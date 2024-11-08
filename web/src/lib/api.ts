import zod from "zod"

const apiBaseUrl = "/api"

const minecraftPattern = zod.object({
    pattern: zod.string(),
    color: zod.string(),
})

export type MinecraftPattern = zod.infer<typeof minecraftPattern>

const metaResponse = zod.object({
    color: zod.string(),
    patterns: zod.array(minecraftPattern),
})

export type MetaResponse = zod.infer<typeof metaResponse>

export const apiUrl = (url: string) =>
    `${apiBaseUrl}${url}`

export const getBannerMeta = async (banner: string): Promise<MetaResponse> => {
    try {
        const response = await fetch(apiUrl(`/v1/meta/${banner}`))
        const json = await response.json()

        return metaResponse.parse(json)
    } catch (e) {
        console.error("Failed to fetch banner meta: ", e)
        throw e
    }
}
