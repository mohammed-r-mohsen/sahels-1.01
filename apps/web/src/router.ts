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

      <section class="grid gap-6 md:grid-cols-3">
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
          <div class="text-sm text-slate-400">وقت الرد</div>
          <div class="mt-3 text-3xl font-black text-white">03:12</div>
          <div class="mt-2 text-sm text-cyan-400">واتساب + SMS + Instagram</div>
        </div>
      </section>

      <section class="mt-8 grid gap-6 lg:grid-cols-[1.6fr_0.9fr]">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div class="mb-4 flex items-center justify-between">
            <h2 class="text-xl font-bold">أداء العيادة</h2>
            <span class="rounded-full border border-brand-500/30 bg-brand-500/10 px-3 py-1 text-xs text-brand-200">Operational</span>
          </div>

          <div class="space-y-5">
            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>تسليم التذكيرات</span>
                <span>91%</span>
              </div>
              <div class="h-2.5 rounded-full bg-slate-800">
                <div class="h-2.5 w-[91%] rounded-full bg-brand-500"></div>
              </div>
            </div>
            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>تأكيد الحجز</span>
                <span>87%</span>
              </div>
              <div class="h-2.5 rounded-full bg-slate-800">
                <div class="h-2.5 w-[87%] rounded-full bg-cyan-500"></div>
              </div>
            </div>
            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>معدل المتابعة</span>
                <span>76%</span>
              </div>
              <div class="h-2.5 rounded-full bg-slate-800">
                <div class="h-2.5 w-[76%] rounded-full bg-amber-500"></div>
              </div>
            </div>
          </div>
        </div>

        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-xl font-bold">طبقة الأمان</h2>
          <ul class="mt-5 space-y-3 text-sm text-slate-300">
            <li>• JWT + clinic-level auth</li>
            <li>• OTP validation</li>
            <li>• rate limiting</li>
            <li>• Argon2 password hashing</li>
            <li>• encrypted secret storage</li>
            <li>• audit logs and backups</li>
          </ul>
        </div>
      </section>

      <section class="mt-8 rounded-2xl border border-slate-800 bg-slate-900 p-5">
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-xl font-bold">المواعيد القادمة</h2>
          <button class="rounded-lg border border-brand-500/40 bg-brand-500/10 px-3 py-2 text-sm text-brand-200">+ حجز جديد</button>
        </div>

        <div class="overflow-x-auto">
          <table class="min-w-full text-right text-sm">
            <thead class="text-slate-400">
              <tr>
                <th class="py-3 pr-4">المريض</th>
                <th class="py-3 pr-4">التاريخ</th>
                <th class="py-3 pr-4">الوقت</th>
                <th class="py-3 pr-4">الطبيب</th>
                <th class="py-3 pr-4">الحالة</th>
              </tr>
            </thead>
            <tbody>
              <tr class="border-t border-slate-800">
                <td class="py-3 pr-4">أحمد السلمان</td>
                <td class="py-3 pr-4">2026-10-22</td>
                <td class="py-3 pr-4">09:30</td>
                <td class="py-3 pr-4">د. علي الحربي</td>
                <td class="py-3 pr-4"><span class="rounded-full bg-emerald-500/15 px-2 py-1 text-emerald-300">مؤكد</span></td>
              </tr>
              <tr class="border-t border-slate-800">
                <td class="py-3 pr-4">سارة القحطاني</td>
                <td class="py-3 pr-4">2026-10-22</td>
                <td class="py-3 pr-4">11:00</td>
                <td class="py-3 pr-4">د. مها الزهراني</td>
                <td class="py-3 pr-4"><span class="rounded-full bg-amber-500/15 px-2 py-1 text-amber-300">قيد التأكيد</span></td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="mt-8 grid gap-6 md:grid-cols-2">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-xl font-bold">تكاملات العيادة</h2>
          <ul class="mt-5 space-y-3 text-sm text-slate-300">
            <li>• Swift payment token</li>
            <li>• WhatsApp Business integration</li>
            <li>• Instagram messaging access</li>
            <li>• clinic-scoped API token</li>
          </ul>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <h2 class="text-xl font-bold">نسخ احتياطية</h2>
          <ul class="mt-5 space-y-3 text-sm text-slate-300">
            <li>• Retention policy: 30 days</li>
            <li>• Daily and manual backups</li>
            <li>• Restore automation</li>
            <li>• Audit and access log support</li>
          </ul>
        </div>
      </section>
    </div>
  </div>
</template>
