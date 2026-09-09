import "./style.css";
import dictionnaireRaw from '@/data/noun.csv?raw'
import { createApp } from "vue";
import App from "./App.vue";


createApp(App).mount("#app");

const lignes = dictionnaireRaw.split('\n').map(l => l.trim()).filter(Boolean)
const mots = lignes[0].toLowerCase() === 'mot' ? lignes.slice(1) : lignes

const dictionnaire = new Set(mots.map(m => m.toLowerCase()))

export function estUnMotValide(mot: string): boolean {
  return dictionnaire.has(mot.toLowerCase())
}