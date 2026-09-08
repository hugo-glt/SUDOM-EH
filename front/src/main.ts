import dictionnaireRaw from '@/data/noun.csv?raw';

const lignes = dictionnaireRaw.split('\n').map(l => l.trim()).filter(Boolean)

// Si la première ligne est un en-tête (ex: "mot"), on l'enlève :
const mots = lignes[0].toLowerCase() === 'mot' ? lignes.slice(1) : lignes

const dictionnaire = new Set(mots.map(m => m.toLowerCase()))

export function estUnMotValide(mot: string): boolean {
  return dictionnaire.has(mot.toLowerCase())
}