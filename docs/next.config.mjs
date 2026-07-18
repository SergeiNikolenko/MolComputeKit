import nextra from 'nextra'

const withNextra = nextra({
  defaultShowCopyCode: true,
  search: { codeblocks: false }
})

const config = withNextra({
  reactStrictMode: true
})

// Nextra 4.2 emits the pre-15.3 `experimental.turbo` key. Next.js 15.5
// accepts the same configuration at the stable top-level `turbopack` key.
const { turbo, ...experimental } = config.experimental ?? {}

export default {
  ...config,
  experimental,
  ...(turbo ? { turbopack: turbo } : {})
}
