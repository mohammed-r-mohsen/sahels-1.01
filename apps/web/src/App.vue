<template>
  <div class="min-h-screen bg-slate-950 text-slate-100">
    <header class="border-b border-slate-800 bg-slate-900/80 backdrop-blur">
      <div class="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
        <div class="text-right">
          <div class="text-xl font-black text-brand-400">Sahels</div>
          <div class="text-xs text-slate-400">Clinic Operations Platform</div>
        </div>
        <nav class="hidden items-center gap-6 text-sm text-slate-300 md:flex">
          <router-link to="/dashboard" class="hover:text-white">لوحة التحكم</router-link>
          <router-link to="/login" class="hover:text-white">تسجيل الدخول</router-link>
          <router-link to="/register" class="hover:text-white">إنشاء حساب</router-link>
        </nav>
      </div>
    </header>

    <main class="mx-auto max-w-7xl px-6 py-8">
      <router-view />
    </main>
  </div>
</template>
