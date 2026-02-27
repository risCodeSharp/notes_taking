// fileData.ts
import type { FileNode } from '@/types'

export const files: FileNode[] = [
  {
    key: '0',
    label: 'src',
    icon: 'pi pi-folder',
    expandedIcon: 'pi pi-folder-open',
    collapsedIcon: 'pi pi-folder',
    children: [
      {
        key: '0-0',
        label: 'components',
        icon: 'pi pi-folder',
        expandedIcon: 'pi pi-folder-open',
        collapsedIcon: 'pi pi-folder',
        children: [
          {
            key: '0-0-0',
            label: 'Header.vue',
            icon: 'pi pi-file'
          },
          {
            key: '0-0-1',
            label: 'Sidebar.vue',
            icon: 'pi pi-file'
          }
        ]
      },
      {
        key: '0-1',
        label: 'App.vue',
        icon: 'pi pi-file'
      }
    ]
  },
  {
    key: '1',
    label: 'package.json',
    icon: 'pi pi-file'
  }
]