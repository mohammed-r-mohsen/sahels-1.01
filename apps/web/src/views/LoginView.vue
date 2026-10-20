<template>
  <div class="min-h-screen bg-slate-950 text-white">
    <div class="mx-auto flex min-h-screen max-w-7xl items-center justify-center px-6 py-10">
      <div class="grid w-full max-w-6xl overflow-hidden rounded-3xl border border-slate-800 bg-slate-900/80 shadow-2xl shadow-slate-950/40 lg:grid-cols-[1.1fr_0.9fr]">
        <div class="relative hidden bg-gradient-to-br from-brand-600 via-brand-700 to-slate-900 p-10 lg:block">
          <div class="absolute inset-0 bg-[radial-gradient(circle_at_top_left,_rgba(255,255,255,0.18),transparent_40%)]" />
          <div class="relative z-10 flex h-full flex-col justify-between">
            <div>
              <div class="text-2xl font-black tracking-wide">Sahels</div>
              <div class="mt-3 text-sm text-slate-200/90">منصة إدارة عيادات سعودية</div>
            </div>
            <div class="space-y-5">
              <h1 class="text-4xl font-black leading-tight">نظام إدارة المواعيد وذكاء التشغيل للعيادات</h1>
              <p class="max-w-md text-sm text-slate-200/80">
                تذكير تلقائي، إدارة مواعيد ذكية، تقارير أداء، ومراقبة عدم الحضور مع نظام أمان متقدم.
              </p>
              <ul class="space-y-2 text-sm text-slate-100/90">
                <li>• إدارة حجوزات المرضى</li>
                <li>• تذكير واتساب + SMS + إنستغرام</li>
                <li>• تبليغ no-show وتوقعية الحضور</li>
                <li>• OCR/AI-ready framework</li>
              </ul>
            </div>
            <div class="text-xs text-slate-200/80">Saudi clinic operations · secure by design</div>
          </div>
        </div>

        <div class="flex items-center justify-center p-8 lg:p-12">
          <div class="w-full max-w-md">
            <div class="mb-8 text-right">
              <div class="text-sm text-brand-400">تسجيل الدخول</div>
              <h2 class="mt-1 text-3xl font-black text-white">لوحة العيادة</h2>
            </div>

            <form class="space-y-5" @submit.prevent="onSubmit">
              <div class="text-right">
                <label class="mb-2 block text-sm text-slate-300">معرف العيادة</label>
                <input v-model="clinicId" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white placeholder:text-slate-500 focus:border-brand-500 focus:outline-none" placeholder="clinic_id" />
              </div>

              <div class="text-right">
                <label class="mb-2 block text-sm text-slate-300">البريد الإلكتروني</label>
                <input v-model="email" type="email" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white placeholder:text-slate-500 focus:border-brand-500 focus:outline-none" placeholder="admin@clinic.sa" />
              </div>

              <div class="text-right">
                <label class="mb-2 block text-sm text-slate-300">كلمة المرور</label>
                <input v-model="password" type="password" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white placeholder:text-slate-500 focus:border-brand-500 focus:outline-none" placeholder="••••••••" />
              </div>

              <div class="flex items-center justify-between text-sm text-slate-400">
                <label class="flex items-center gap-2">
                  <input type="checkbox" class="rounded border-slate-700 bg-slate-800" />
                  تذكرني
                </label>
                <a href="#" class="text-brand-400 hover:text-brand-300">نسيت كلمة المرور؟</a>
              </div>

              <button type="submit" class="w-full rounded-xl bg-brand-500 px-4 py-3 font-bold text-white transition hover:bg-brand-400">
                تسجيل الدخول
              </button>
            </form>

            <div class="mt-6 text-center text-sm text-slate-400">
              لا يوجد حساب؟
              <router-link to="/register" class="font-semibold text-brand-400">إنشاء حساب جديد</router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const clinicId = ref('test_clinic');
const email = ref('admin@clinic.sa');
const password = ref('');

const onSubmit = () => {
  router.push('/dashboard');
};
</script>
