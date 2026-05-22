// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

// stores/theme.ts
import "@catppuccin/tailwindcss/mocha.css";
import { onMounted, ref } from "vue";

export type Flavour = "mocha" | "frappe" | "macchiato" | "latte";
export type Accent =
  | "rosewater"
  | "flamingo"
  | "pink"
  | "mauve"
  | "red"
  | "maroon"
  | "peach"
  | "yellow"
  | "green"
  | "teal"
  | "sky"
  | "sapphire"
  | "blue"
  | "lavender";

const currentFlavour = ref<Flavour>("mocha");
const currentAccent = ref<Accent>("blue");

const accentColours = [
  "rosewater",
  "flamingo",
  "pink",
  "mauve",
  "red",
  "maroon",
  "peach",
  "yellow",
  "green",
  "teal",
  "sky",
  "sapphire",
  "blue",
  "lavender",
] as const;

export const useTheme = () => {
  const setFlavour = async (flavour: Flavour) => {
    currentFlavour.value = flavour;
    document.documentElement.classList.remove(
      "latte",
      "frappe",
      "macchiato",
      "mocha"
    );
    document.documentElement.classList.add(flavour);
    localStorage.setItem("theme-flavour", flavour);
  };

  const setAccent = async (accent: Accent) => {
    currentAccent.value = accent;
    document.documentElement.style.setProperty(
      "--ctp-accent",
      `var(--ctp-${accent})`
    );
    localStorage.setItem("theme-accent", accent);
  };

  const hydrate = async () => {
    const savedFlavour = localStorage.getItem(
      "theme-flavour"
    ) as Flavour | null;
    if (savedFlavour) await setFlavour(savedFlavour);

    const savedAccent = localStorage.getItem("theme-accent") as Accent | null;
    if (savedAccent) await setAccent(savedAccent);
  };

  onMounted(() => hydrate());

  return {
    currentFlavour,
    currentAccent,
    setFlavour,
    setAccent,
    flavours: ["latte", "frappe", "macchiato", "mocha"] as const,
    accentColours,
  };
};
