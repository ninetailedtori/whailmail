import { defineStore } from "pinia";
import { ref } from "vue";

export type CatppuccinFlavor = "latte" | "frappe" | "macchiato" | "mocha";

const FLAVORS: CatppuccinFlavor[] = ["latte", "frappe", "macchiato", "mocha"];
const STORAGE_KEY = "ctp-flavor";

export const useThemeStore = defineStore("theme", () => {
  const currentFlavor = ref<CatppuccinFlavor>("macchiato");

  const loadTheme = (): void => {
    const saved = localStorage.getItem(STORAGE_KEY) as CatppuccinFlavor | null;
    if (saved && FLAVORS.includes(saved)) {
      currentFlavor.value = saved;
    }
    applyTheme(currentFlavor.value);
  };

  const applyTheme = (flavor: CatppuccinFlavor): void => {
    const html = document.documentElement;
    FLAVORS.forEach((f) => html.classList.remove(`ctp-${f}`));
    html.classList.add(`ctp-${flavor}`);
    localStorage.setItem(STORAGE_KEY, flavor);
  };

  const setFlavor = (flavor: CatppuccinFlavor): void => {
    currentFlavor.value = flavor;
    applyTheme(flavor);
  };

  return {
    currentFlavor,
    flavors: FLAVORS,
    setFlavor,
    loadTheme,
  };
});
