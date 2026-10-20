<template>
  <div class="min-h-screen bg-slate-950 px-6 py-10 text-white">
    <div class="mx-auto max-w-2xl rounded-3xl border border-slate-800 bg-slate-900 p-8 shadow-2xl shadow-slate-950/40">
      <div class="mb-8 text-right">
        <div class="text-sm text-brand-400">إنشاء حساب</div>
        <h2 class="mt-1 text-3xl font-black">تسجيل عيادة جديدة</h2>
      </div>

      <form class="space-y-5 text-right" @submit.prevent="onSubmit">
        <div class="grid gap-5 md:grid-cols-2">
          <div>
            <label class="mb-2 block text-sm text-slate-300">اسم العيادة</label>
            <input v-model="clinicName" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="عيادة النخيل" />
          </div>
          <div>
            <label class="mb-2 block text-sm text-slate-300">معرف العيادة</label>
            <input v-model="clinicId" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="clinic_id" />
          </div>
        </div>

        <div class="grid gap-5 md:grid-cols-2">
          <div>
            <label class="mb-2 block text-sm text-slate-300">رقم الهاتف</label>
            <input v-model="phone" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="+966500000000" />
          </div>
          <div>
            <label class="mb-2 block text-sm text-slate-300">البريد الإلكتروني</label>
            <input v-model="email" type="email" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="info@clinic.sa" />
          </div>
        </div>

        <div class="grid gap-5 md:grid-cols-2">
          <div>
            <label class="mb-2 block text-sm text-slate-300">كلمة المرور</label>
            <input v-model="password" type="password" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="••••••••" />
          </div>
          <div>
            <label class="mb-2 block text-sm text-slate-300">تأكيد كلمة المرور</label>
            <input v-model="confirmPassword" type="password" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-right text-white focus:border-brand-500 focus:outline-none" placeholder="••••••••" />
          </div>
        </div>

        <div class="rounded-2xl border border-brand-500/20 bg-brand-500/10 p-4 text-right text-sm text-brand-200">
          <div class="font-bold">معلومات التكامل</div>
          <div class="mt-2">سيتم تفعيل: دفع سنوي/شهري، واتساب، إنستغرام، OTP، and API token</div>
        </div>

        <button type="submit" class="w-full rounded-xl bg-brand-500 px-4 py-3 font-bold text-white transition hover:bg-brand-400">
          إنشاء الحساب
        </button>
      </form>

      <div class="mt-6 text-center text-sm text-slate-400">
        لديك حساب بالفعل؟
        <router-link to="/login" class="font-semibold text-brand-400">تسجيل الدخول</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const clinicName = ref('عيادة النخيل');
const clinicId = ref('test_clinic');
const phone = ref('+966500000000');
const email = ref('admin@clinic.sa');
const password = ref('');
const confirmPassword = ref('');

const onSubmit = () => {
  router.push('/dashboard');
};
</script>
