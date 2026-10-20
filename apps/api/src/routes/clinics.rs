<template>
  <div class="min-h-screen bg-slate-950 p-6 text-slate-100">
    <div class="mx-auto max-w-7xl">
      <header class="mb-8 flex flex-col gap-4 rounded-2xl border border-slate-800 bg-slate-900 p-5 md:flex-row md:items-center md:justify-between">
        <div class="text-right">
          <div class="text-sm text-brand-400">لوحة التحكم</div>
          <h1 class="text-3xl font-black">Sahels Dashboard</h1>
        </div>
        <div class="rounded-xl border border-slate-700 bg-slate-800 px-4 py-2 text-sm text-slate-300">
          العيادة: <span class="text-white">عيادة النخيل</span>
        </div>
      </header>

      <section class="grid gap-6 md:grid-cols-4">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="text-sm text-slate-400">إجمالي المواعيد اليوم</div>
          <div class="mt-3 text-3xl font-black text-white">184</div>
          <div class="mt-2 text-sm text-emerald-400">+12% من الأسبوع الماضي</div>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="text-sm text-slate-400">نسبة عدم الحضور</div>
          <div class="mt-3 text-3xl font-black text-white">6.2%</div>
          <div class="mt-2 text-sm text-amber-400">تم تقليلها بنسبة 18%</div>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="text-sm text-slate-400">الأطباء المتواجدون</div>
          <div class="mt-3 text-3xl font-black text-white">4</div>
          <div class="mt-2 text-sm text-cyan-400">من أصل 6 أطباء</div>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="text-sm text-slate-400">العربون المفعّل</div>
          <div class="mt-3 text-3xl font-black text-white">50 ر.س</div>
          <div class="mt-2 text-sm text-violet-400">مفعّل لكل حجز</div>
        </div>
      </section>

      <section class="mt-8 grid gap-6 lg:grid-cols-[1.5fr_1fr]">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="mb-4 flex items-center justify-between">
            <h2 class="text-xl font-bold">إدارة الأطباء</h2>
            <span class="rounded-full border border-brand-500/30 bg-brand-500/10 px-3 py-1 text-xs text-brand-200">Clinic Control</span>
          </div>

          <div class="space-y-4">
            <div class="grid gap-4 md:grid-cols-2">
              <div>
                <label class="mb-2 block text-sm text-slate-300">اسم الطبيب</label>
                <input v-model="doctorName" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white" placeholder="د. علي الحربي" />
              </div>
              <div>
                <label class="mb-2 block text-sm text-slate-300">التخصص</label>
                <input v-model="doctorSpecialty" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white" placeholder="جلدية" />
              </div>
            </div>

            <div class="grid gap-4 md:grid-cols-2">
              <div>
                <label class="mb-2 block text-sm text-slate-300">عدد المرضى في كل فترة</label>
                <input v-model="maxPatients" type="number" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white" value="5" />
              </div>
              <div>
                <label class="mb-2 block text-sm text-slate-300">الحالة</label>
                <select v-model="doctorStatus" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white">
                  <option value="available">متاح</option>
                  <option value="busy">مشغول</option>
                  <option value="off">غير متاح</option>
                </select>
              </div>
            </div>

            <button @click="saveDoctor" class="w-full rounded-xl bg-brand-500 px-4 py-3 font-bold text-white transition hover:bg-brand-400">
              حفظ بيانات الطبيب
            </button>
          </div>
        </div>

        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-xl font-bold">دفع العربون</h2>
          <div class="mt-5 space-y-4">
            <div>
              <label class="mb-2 block text-sm text-slate-300">طريقة الدفع</label>
              <select v-model="paymentProvider" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white">
                <option value="moyasar">Moyasar</option>
                <option value="tabby">Tabby</option>
                <option value="cash">نقدي</option>
                <option value="bank">تحويل بنكي</option>
              </select>
            </div>

            <div>
              <label class="mb-2 block text-sm text-slate-300">قيمة العربون</label>
              <input v-model="depositAmount" type="number" class="w-full rounded-xl border border-slate-700 bg-slate-800 px-3 py-2 text-right text-white" value="50" />
            </div>

            <label class="flex items-center justify-between rounded-xl border border-slate-700 bg-slate-800 px-3 py-3 text-sm text-slate-300">
              <span>إلزام العميل بدفع العربون قبل التأكيد</span>
              <input v-model="depositRequired" type="checkbox" class="h-4 w-4 rounded border-slate-600 bg-slate-700" />
            </label>

            <button @click="savePaymentMethod" class="w-full rounded-xl bg-emerald-500 px-4 py-3 font-bold text-white transition hover:bg-emerald-400">
              حفظ طريقة الدفع
            </button>
          </div>
        </div>
      </section>

      <section class="mt-8 rounded-2xl border border-slate-800 bg-slate-900 p-5">
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-xl font-bold">الأطباء الحاليون</h2>
          <span class="text-sm text-slate-400">إدارة حضور ووقت العمل</span>
        </div>

        <div class="overflow-x-auto">
          <table class="min-w-full text-right text-sm">
            <thead class="text-slate-400">
              <tr>
                <th class="py-3 pr-4">الطبيب</th>
                <th class="py-3 pr-4">التخصص</th>
                <th class="py-3 pr-4">الحد الأقصى يوميًا</th>
                <th class="py-3 pr-4">الحالة</th>
                <th class="py-3 pr-4">مواعيد العمل</th>
              </tr>
            </thead>
            <tbody>
              <tr class="border-t border-slate-800">
                <td class="py-3 pr-4">د. علي الحربي</td>
                <td class="py-3 pr-4">جلدية</td>
                <td class="py-3 pr-4">8</td>
                <td class="py-3 pr-4"><span class="rounded-full bg-emerald-500/15 px-2 py-1 text-emerald-300">متاح</span></td>
                <td class="py-3 pr-4">الاثنين / الثلاثاء / الأربعاء</td>
              </tr>
              <tr class="border-t border-slate-800">
                <td class="py-3 pr-4">د. مها الزهراني</td>
                <td class="py-3 pr-4">الأسنان</td>
                <td class="py-3 pr-4">6</td>
                <td class="py-3 pr-4"><span class="rounded-full bg-cyan-500/15 px-2 py-1 text-cyan-300">متاح</span></td>
                <td class="py-3 pr-4">الأحد / الثلاثاء / الخميس</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const doctorName = ref('د. علي الحربي');
const doctorSpecialty = ref('جلدية');
const maxPatients = ref(5);
const doctorStatus = ref('available');
const paymentProvider = ref('moyasar');
const depositAmount = ref(50);
const depositRequired = ref(true);

const saveDoctor = () => {
  console.log('doctor settings saved', {
    doctorName: doctorName.value,
    doctorSpecialty: doctorSpecialty.value,
    maxPatients: maxPatients.value,
    doctorStatus: doctorStatus.value,
  });
};

const savePaymentMethod = () => {
  console.log('payment method saved', {
    paymentProvider: paymentProvider.value,
    depositAmount: depositAmount.value,
    depositRequired: depositRequired.value,
  });
};
</script>
