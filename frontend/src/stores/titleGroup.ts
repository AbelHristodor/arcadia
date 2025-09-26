import type { TitleGroupLite } from '@/services/api/torrentService'
import { defineStore } from 'pinia'

export const useTitleGroupStore = defineStore('titleGroup', {
  // consider either making store type nullish or specifying well known default placeholder values
  state: (): TitleGroupLite => {
    return {
      id: 0,
      content_type: 'movie',
      edition_groups: [],
      name: '',
      covers: [],
      original_release_date: '',
    }
  },
})
