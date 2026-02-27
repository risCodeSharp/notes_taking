export type Mode = "Edit" | "Preview" | "Split";


export interface FileNode {
  key: string
  label: string
  icon?: string
  expandedIcon?: string
  collapsedIcon?: string
  children?: FileNode[]
}