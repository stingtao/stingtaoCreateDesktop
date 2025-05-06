import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import SettingsView from '../views/Settings.vue'
import HelpView from '../views/Help.vue'
import NewProject from '../views/NewProject.vue'
import CreateBookProject from '../views/CreateBookProject.vue'
import CreateBlogProject from '../views/CreateBlogProject.vue'
import Projects from '../views/Projects.vue'
import EditProject from '../views/EditProject.vue'
import NewBlogArticle from '../components/blog-article/NewBlogArticle.vue'
import BookChapterDraft from '../views/BookChapterDraft.vue'
import ReviewChapters from '../views/ReviewChapters.vue'
import EditChapter from '../views/EditChapter.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView
    },
    {
      path: '/help',
      name: 'help',
      component: HelpView
    },
    {
      path: '/projects',
      name: 'projects',
      component: Projects
    },
    {
      path: '/projects/:id',
      name: 'edit-project',
      component: EditProject
    },
    {
      path: '/new-project',
      name: 'new-project',
      component: NewProject
    },
    {
      path: '/create-book-project',
      name: 'create-book-project',
      component: CreateBookProject
    },
    {
      path: '/create-blog-project',
      name: 'create-blog-project',
      component: CreateBlogProject
    },
    {
      path: '/new-blog-article',
      name: 'new-blog-article',
      component: NewBlogArticle,
      props: (route) => ({
        key: `${route.query.project_id}-${route.query.blog_id}`
      })
    },
    {
      path: '/projects/:project_id/articles/new',
      name: 'new-project-article',
      component: NewBlogArticle,
      props: (route) => ({
        key: `${route.params.project_id}-new`
      })
    },
    {
      path: '/projects/:project_id/articles/:blog_id',
      name: 'edit-blog-article',
      component: NewBlogArticle,
      props: (route) => ({
        key: `${route.params.project_id}-${route.params.blog_id}`
      })
    },
    {
      path: '/new-chapter',
      name: 'new-chapter',
      component: BookChapterDraft
    },
    {
      path: '/review-chapters',
      name: 'review-chapters',
      component: ReviewChapters
    },
    {
      path: '/projects/:project_id/chapters/:chapter_id',
      name: 'edit-chapter',
      component: EditChapter
    }
  ]
})

export default router 