import { chromium } from "npm:@playwright/test";

async function getExample(
  year: string,
  day: string,
): Promise<string> {
  const browser = await chromium.launch();
  const context = await browser.newContext();
  const page = await context.newPage();

  const url = `https://adventofcode.com/${year}/day/${day}`;
  await page.goto(url);

  const codeBlocks = await page.locator("pre code").all();

  let example = "";
  const exampleElement = codeBlocks[0];
  if (exampleElement) {
    example = await exampleElement.innerText();
  }

  await browser.close();
  return example.trim();
}

async function run() {
  const args = Deno.args;

  if (args.length < 2) {
    console.error("Usage: deno run main.ts <year> <day>");
    Deno.exit(1);
  }

  const [year, day] = args;

  try {
    const example = await getExample(year, day);
    console.log(example);
  } catch (error) {
    console.error("Error fetching example:", error);
    Deno.exit(1);
  }
}

if (import.meta.main) {
  await run();
}
