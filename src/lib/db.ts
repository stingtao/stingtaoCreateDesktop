// 定義專案類型
export interface Project {
  id?: number;
  title: string;
  type_: string;
  description?: string;
  category?: string;
  time_commitment?: string;
  publishing_frequency?: string;
  custom_frequency?: string;
  deadline?: string;
  availability?: string;
  reminder_frequency?: string;
  publishing_platform?: string;
  wordpress_url?: string;
  substack_url?: string;
  custom_platform?: string;
  monetization_strategy?: string;
  monetization_goals?: string;
  keywords?: string;
  target_audience?: string;
  reference_links?: string;
  structure?: string;
  content_strategy?: string;
  seo_strategy?: string;
  goal?: string;
  start_date?: string;
  end_date?: string;
  created_at?: string;
  updated_at?: string;
  progress?: number;
}

// 定義內容類型
export interface Content {
  id: number;
  project_id: number;
  title: string;
  content: string;
  order: number;
  created_at: string;
  updated_at: string;
}

// 定義設定類型
export interface Settings {
  id: number;
  key: string;
  value: string;
  created_at: string;
  updated_at: string;
} 