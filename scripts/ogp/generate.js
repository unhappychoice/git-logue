import satori from 'satori';
import { Resvg } from '@resvg/resvg-js';
import fs from 'fs/promises';
import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function fetchFontFromGoogleFonts(fontFamily) {
  const cssUrl = `https://fonts.googleapis.com/css2?family=${fontFamily.replace(/\s+/g, '+')}:wght@700&display=swap`;
  const cssResponse = await fetch(cssUrl);
  const cssText = await cssResponse.text();

  const match = cssText.match(/url\((https:\/\/fonts\.gstatic\.com\/[^)]+\.ttf)\)/);
  if (!match) {
    throw new Error(`Could not find font URL for ${fontFamily}`);
  }

  const fontUrl = match[1];
  const fontResponse = await fetch(fontUrl);
  const arrayBuffer = await fontResponse.arrayBuffer();
  return Buffer.from(arrayBuffer);
}

async function generateOGP() {
  const width = 1200;
  const height = 630;

  // Download fonts from Google Fonts
  console.log('Downloading Crimson Text...');
  const crimsonTextData = await fetchFontFromGoogleFonts('Crimson Text');
  console.log('Downloading Lora...');
  const loraData = await fetchFontFromGoogleFonts('Lora');

  // Code snippet with syntax highlighting
  const codeLines = [
    { num: '174', code: '    ', color: '#E5E5E5' },
    { num: '175', code: '    pub fn new(config: Config) -> Self {', color: '#E5E5E5' },
    { num: '176', code: '        Self { engine: Engine::new(config) }', color: '#E5E5E5' },
    { num: '177', code: '    }', color: '#E5E5E5' },
    { num: '178', code: '    ', color: '#E5E5E5' },
    { num: '179', code: '-   pub fn load(&mut self, meta: Metadata) {', color: '#E06C75', bg: '#3F1F1F' },
    { num: '180', code: '+   pub fn load(&mut self, meta: Metadata) -> Result<()> {', color: '#89E051', bg: '#1F3F1F' },
    { num: '181', code: '        self.metadata = Some(meta.clone());', color: '#E5E5E5' },
    { num: '182', code: '        self.engine.load(meta)?;', color: '#E5E5E5' },
    { num: '183', code: '+       self.validate_state()?;', color: '#89E051', bg: '#1F3F1F' },
    { num: '184', code: '        Ok(())', color: '#E5E5E5' },
    { num: '185', code: '    }', color: '#E5E5E5' },
    { num: '186', code: '    ', color: '#E5E5E5' },
    { num: '187', code: '    pub fn render(&mut self) -> Result<()> {', color: '#E5E5E5' },
    { num: '188', code: '        self.engine.render()', color: '#E5E5E5' },
  ];

  const svg = await satori(
    {
      type: 'div',
      props: {
        style: {
          width: '100%',
          height: '100%',
          display: 'flex',
          backgroundColor: '#0C0C0F',
        },
        children: [
          // Left side: Title and info (40%)
          {
            type: 'div',
            props: {
              style: {
                width: '40%',
                height: '100%',
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
                alignItems: 'flex-start',
                padding: '40px',
                gap: 30,
              },
              children: [
                // Title
                {
                  type: 'div',
                  props: {
                    style: {
                      fontSize: 76,
                      fontWeight: 700,
                      fontFamily: 'Crimson Text',
                      color: '#E5E5E5',
                      letterSpacing: '-0.02em',
                    },
                    children: 'gitlogue',
                  },
                },
                // Tagline
                {
                  type: 'div',
                  props: {
                    style: {
                      fontSize: 24,
                      fontFamily: 'Lora',
                      color: '#A0A0A0',
                      lineHeight: 1.5,
                    },
                    children: 'Cinematic Git commit replay for your terminal',
                  },
                },
                // Subcopy
                {
                  type: 'div',
                  props: {
                    style: {
                      fontSize: 18,
                      fontFamily: 'Lora',
                      color: '#61AFEF',
                      fontStyle: 'italic',
                    },
                    children: 'Watch your code history come alive.',
                  },
                },
                // Use cases
                {
                  type: 'div',
                  props: {
                    style: {
                      display: 'flex',
                      flexDirection: 'column',
                      gap: 6,
                      marginTop: 30,
                    },
                    children: [
                      {
                        type: 'div',
                        props: {
                          style: {
                            fontSize: 16,
                            fontFamily: 'Lora',
                            display: 'flex',
                            gap: 6,
                          },
                          children: [
                            { type: 'div', props: { style: { color: '#7AA2F7' }, children: '▸ Screensaver' } },
                            { type: 'div', props: { style: { color: '#565F89' }, children: '— Ambient coding display' } },
                          ],
                        },
                      },
                      {
                        type: 'div',
                        props: {
                          style: {
                            fontSize: 16,
                            fontFamily: 'Lora',
                            display: 'flex',
                            gap: 6,
                          },
                          children: [
                            { type: 'div', props: { style: { color: '#9ECE6A' }, children: '▸ Education' } },
                            { type: 'div', props: { style: { color: '#565F89' }, children: '— Visualize code evolution' } },
                          ],
                        },
                      },
                      {
                        type: 'div',
                        props: {
                          style: {
                            fontSize: 16,
                            fontFamily: 'Lora',
                            display: 'flex',
                            gap: 6,
                          },
                          children: [
                            { type: 'div', props: { style: { color: '#E0AF68' }, children: '▸ Presentations' } },
                            { type: 'div', props: { style: { color: '#565F89' }, children: '— Replay commit histories' } },
                          ],
                        },
                      },
                      {
                        type: 'div',
                        props: {
                          style: {
                            fontSize: 16,
                            fontFamily: 'Lora',
                            display: 'flex',
                            gap: 6,
                          },
                          children: [
                            { type: 'div', props: { style: { color: '#BB9AF7' }, children: '▸ Content Creation' } },
                            { type: 'div', props: { style: { color: '#565F89' }, children: '— Record with VHS/asciinema' } },
                          ],
                        },
                      },
                      {
                        type: 'div',
                        props: {
                          style: {
                            fontSize: 16,
                            fontFamily: 'Lora',
                            display: 'flex',
                            gap: 6,
                          },
                          children: [
                            { type: 'div', props: { style: { color: '#7DCFFF' }, children: '▸ Desktop Ricing' } },
                            { type: 'div', props: { style: { color: '#565F89' }, children: '— Living terminal decoration' } },
                          ],
                        },
                      },
                    ],
                  },
                },
                // GitHub URL
                {
                  type: 'div',
                  props: {
                    style: {
                      fontSize: 16,
                      fontFamily: 'Lora',
                      color: '#4B5263',
                      marginTop: 'auto',
                    },
                    children: 'github.com/unhappychoice/gitlogue',
                  },
                },
              ],
            },
          },
          // Right side: Terminal box with TUI layout (60%)
          {
            type: 'div',
            props: {
              style: {
                width: '60%',
                height: '100%',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                padding: '30px',
                backgroundColor: '#0C0C0F',
              },
              children: [
                // Terminal box wrapper
                {
                  type: 'div',
                  props: {
                    style: {
                      width: '100%',
                      height: '100%',
                      backgroundColor: '#1A1B26',
                      borderRadius: 8,
                      overflow: 'hidden',
                      display: 'flex',
                      flexDirection: 'column',
                    },
                    children: [
                      // Top row: FileTree | Editor
                      {
                        type: 'div',
                        props: {
                          style: {
                            display: 'flex',
                            height: '75%',
                          },
                          children: [
                            // FileTree (left)
                            {
                              type: 'div',
                              props: {
                                style: {
                                  width: '25%',
                                  backgroundColor: '#16161E',
                                  padding: '15px',
                                  display: 'flex',
                                  flexDirection: 'column',
                                  gap: 4,
                                  fontFamily: 'JetBrains Mono',
                                  fontSize: 11,
                                  color: '#565F89',
                                },
                                children: [
                                  { type: 'div', props: { style: { color: '#7AA2F7' }, children: 'src/' } },
                                  { type: 'div', props: { style: { color: '#9ECE6A', paddingLeft: 10 }, children: '~ ui.rs +28 -7' } },
                                  { type: 'div', props: { style: { paddingLeft: 10, color: '#565F89' }, children: '  animation.rs' } },
                                  { type: 'div', props: { style: { paddingLeft: 10, color: '#565F89' }, children: '  config.rs' } },
                                  { type: 'div', props: { style: { paddingLeft: 10, color: '#565F89' }, children: '  git.rs' } },
                                  { type: 'div', props: { style: { color: '#7AA2F7', marginTop: 4 }, children: 'Cargo.toml' } },
                                  { type: 'div', props: { style: { color: '#7AA2F7' }, children: 'README.md' } },
                                ],
                              },
                            },
                            // Editor (right)
                            {
                              type: 'div',
                              props: {
                                style: {
                                  width: '75%',
                                  backgroundColor: '#1A1B26',
                                  padding: '15px',
                                  display: 'flex',
                                  flexDirection: 'column',
                                  gap: 1,
                                  fontFamily: 'JetBrains Mono',
                                  fontSize: 11,
                                },
                                children: codeLines.map(line => ({
                                  type: 'div',
                                  props: {
                                    style: {
                                      display: 'flex',
                                      backgroundColor: line.bg || 'transparent',
                                      paddingLeft: 4,
                                      paddingRight: 4,
                                    },
                                    children: [
                                      {
                                        type: 'div',
                                        props: {
                                          style: {
                                            color: '#3B4261',
                                            marginRight: 12,
                                            width: 30,
                                            textAlign: 'right',
                                            fontSize: 10,
                                          },
                                          children: line.num,
                                        },
                                      },
                                      {
                                        type: 'div',
                                        props: {
                                          style: {
                                            color: line.color,
                                            whiteSpace: 'pre',
                                          },
                                          children: line.code,
                                        },
                                      },
                                    ],
                                  },
                                })),
                              },
                            },
                          ],
                        },
                      },
                      // Bottom row: Commit meta | Terminal
                      {
                        type: 'div',
                        props: {
                          style: {
                            display: 'flex',
                            height: '25%',
                            borderTop: '1px solid #3B4261',
                          },
                          children: [
                            // Commit metadata (left)
                            {
                              type: 'div',
                              props: {
                                style: {
                                  width: '25%',
                                  backgroundColor: '#16161E',
                                  padding: '15px',
                                  display: 'flex',
                                  flexDirection: 'column',
                                  gap: 6,
                                  fontFamily: 'JetBrains Mono',
                                  fontSize: 10,
                                  color: '#9AA5CE',
                                },
                                children: [
                                  { type: 'div', props: { style: { color: '#E0AF68' }, children: 'hash: f16f674' } },
                                  { type: 'div', props: { style: { color: '#9AA5CE' }, children: 'author: Yuji Ueki' } },
                                  { type: 'div', props: { style: { color: '#565F89', fontSize: 9 }, children: 'date: 2025-11-09' } },
                                  { type: 'div', props: { style: { color: '#565F89', fontSize: 9 }, children: '      16:51:33' } },
                                  { type: 'div', props: { style: { color: '#7AA2F7', marginTop: 8 }, children: 'feat: implement' } },
                                  { type: 'div', props: { style: { color: '#7AA2F7' }, children: 'input handling' } },
                                ],
                              },
                            },
                            // Terminal (right)
                            {
                              type: 'div',
                              props: {
                                style: {
                                  width: '75%',
                                  backgroundColor: '#1A1B26',
                                  padding: '15px',
                                  display: 'flex',
                                  flexDirection: 'column',
                                  gap: 3,
                                  fontFamily: 'JetBrains Mono',
                                  fontSize: 10,
                                  color: '#565F89',
                                },
                                children: [
                                  { type: 'div', props: { style: { color: '#9ECE6A' }, children: '~ time-travel 2025-11-09 16:51:33' } },
                                  { type: 'div', props: { style: { color: '#9AA5CE' }, children: 'Compressing digital dreams: 100%' } },
                                  { type: 'div', props: { style: { color: '#9AA5CE' }, children: 'Signing with invisible ink: done.' } },
                                  { type: 'div', props: { style: { color: '#9ECE6A', marginTop: 3 }, children: '3a62bb4..3a62bb4  SUCCESS' } },
                                  { type: 'div', props: { style: { color: '#7AA2F7' }, children: 'Arrived at 2025-11-09 16:51:33' } },
                                ],
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
    {
      width,
      height,
      fonts: [
        {
          name: 'Crimson Text',
          data: crimsonTextData,
          weight: 700,
          style: 'normal',
        },
        {
          name: 'Lora',
          data: loraData,
          weight: 400,
          style: 'normal',
        },
        {
          name: 'JetBrains Mono',
          data: await fs.readFile('/home/owner/.local/share/fonts/jetbrains-mono/JetBrainsMono-Regular.ttf'),
          weight: 400,
          style: 'normal',
        },
      ],
    }
  );

  // Convert SVG to PNG
  const resvg = new Resvg(svg);
  const pngData = resvg.render();
  const pngBuffer = pngData.asPng();

  // Save the image
  const outputPath = path.join(__dirname, '../../docs/assets/ogp.png');
  await fs.writeFile(outputPath, pngBuffer);

  console.log(`✅ OGP image generated: ${outputPath}`);
  console.log(`   Size: ${width}x${height}px`);
  console.log(`   "静止しているのに動いて見える"`);
}

generateOGP().catch(console.error);
