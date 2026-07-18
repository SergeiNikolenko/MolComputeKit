import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import { Footer, Layout, Navbar } from 'nextra-theme-docs'
import 'nextra-theme-docs/style.css'

export const metadata = {
  title: {
    default: 'MolComputeKit',
    template: '%s — MolComputeKit'
  },
  description:
    'A practical guide to MolComputeKit CPU references, verified Apple Metal execution, and bounded compute protocols.'
}

const navbar = (
  <Navbar
    logo={<strong>MolComputeKit</strong>}
    projectLink="https://github.com/SergeiNikolenko/MolComputeKit"
    projectIcon={
      <span aria-label="MolComputeKit GitHub repository">GitHub</span>
    }
  />
)
const footer = (
  <Footer>MIT {new Date().getFullYear()} © MolComputeKit contributors.</Footer>
)

export default async function RootLayout({ children }) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head faviconGlyph="M" />
      <body>
        <Layout
          navbar={navbar}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/SergeiNikolenko/MolComputeKit/tree/main/docs"
          footer={footer}
        >
          {children}
        </Layout>
      </body>
    </html>
  )
}
