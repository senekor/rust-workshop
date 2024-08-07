/* ./setup/shiki.ts */
import { defineShikiSetup } from "@slidev/types";

import { transformerNotationWordHighlight, transformerMetaWordHighlight } from "@shikijs/transformers";

export default defineShikiSetup(() => {
  return {
    themes: {
      dark: "dark-plus",
      light: "light-plus",
    },
    transformers: [
      transformerNotationWordHighlight(),
      transformerMetaWordHighlight(),
    ],
  };
});
