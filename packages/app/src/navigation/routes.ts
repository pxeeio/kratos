import {type RouteDefinition} from '@solidjs/router';
import {lazy} from 'solid-js';

import Routes from '@/constants/Routes';

export default [
    {
        path: Routes.Landing.path,
        component: lazy(() => import('@/pages/LandingPage')),
    },
    {
        path: Routes.Login.path,
        component: lazy(() => import('@/pages/LoginPage')),
    },
    {
        path: Routes.Register.path,
        component: lazy(() => import('@/pages/RegisterPage')),
    },
    {
        path: '/app',
        component: lazy(() => import('@/components/AppSiteShell')),
        children: [
            {
                path: Routes.UserDashboard.path,
                component: lazy(() => import('@/pages/app/DashboardPage')),
            },
            {
                path: Routes.Exercises.path,
                component: lazy(() => import('@/pages/app/ExercisesPage')),
            },
            {
                path: Routes.UserTracker.path,
                component: lazy(() => import('@/pages/app/TrackerPage')),
            },
            {
                path: Routes.UserProgress.path,
                component: lazy(() => import('@/pages/app/ProgressPage')),
            },
            {
                path: Routes.UserSettings.path,
                component: lazy(() => import('@/pages/app/SettingsPage')),
            },
            {
                path: '/*',
                component: lazy(() => import('@/pages/Error404Page')),
            },
        ],
    },
    {
        path: '*',
        component: lazy(() => import('@/pages/Error404Page')),
    },
] satisfies RouteDefinition[];
