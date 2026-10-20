<template>
  <div class="min-h-screen bg-slate-950 text-slate-100">
    <header class="border-b border-slate-800 bg-slate-900/80 backdrop-blur">
      <div class="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
        <div>
          <div class="text-xl font-bold text-brand-400">Sahels</div>
          <div class="text-xs text-slate-400">Clinic Operations Platform</div>
        </div>
        <nav class="flex items-center gap-6 text-sm text-slate-300">
          <a href="#" class="hover:text-white">Dashboard</a>
          <a href="#" class="hover:text-white">Appointments</a>
          <a href="#" class="hover:text-white">Patients</a>
          <a href="#" class="hover:text-white">Messages</a>
        </nav>
      </div>
    </header>

    <main class="mx-auto max-w-7xl px-6 py-10">
      <section class="grid gap-6 md:grid-cols-3">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow-lg shadow-slate-950/40">
          <p class="text-sm text-slate-400">Appointments Today</p>
          <p class="mt-3 text-3xl font-bold text-white">184</p>
          <p class="mt-2 text-sm text-emerald-400">+12% vs last week</p>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow-lg shadow-slate-950/40">
          <p class="text-sm text-slate-400">No-Show Rate</p>
          <p class="mt-3 text-3xl font-bold text-white">6.2%</p>
          <p class="mt-2 text-sm text-amber-400">AI alerts reduced by 18%</p>
        </div>
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow-lg shadow-slate-950/40">
          <p class="text-sm text-slate-400">Response Time</p>
          <p class="mt-3 text-3xl font-bold text-white">3m 12s</p>
          <p class="mt-2 text-sm text-cyan-400">Across WhatsApp and SMS</p>
        </div>
      </section>

      <section class="mt-10 grid gap-6 lg:grid-cols-[1.5fr_1fr]">
        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-6">
          <div class="mb-4 flex items-center justify-between">
            <h2 class="text-xl font-semibold">Clinic performance</h2>
            <span class="rounded-full border border-brand-500/30 bg-brand-500/10 px-3 py-1 text-xs text-brand-300">Operational</span>
          </div>

          <div class="space-y-4">
            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>Reminder delivery success</span>
                <span>91%</span>
              </div>
              <div class="h-2 rounded-full bg-slate-800">
                <div class="h-2 w-[91%] rounded-full bg-brand-500"></div>
              </div>
            </div>

            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>Booking confirmation</span>
                <span>87%</span>
              </div>
              <div class="h-2 rounded-full bg-slate-800">
                <div class="h-2 w-[87%] rounded-full bg-cyan-500"></div>
              </div>
            </div>

            <div>
              <div class="mb-2 flex justify-between text-sm text-slate-300">
                <span>Follow-up completion</span>
                <span>76%</span>
              </div>
              <div class="h-2 rounded-full bg-slate-800">
                <div class="h-2 w-[76%] rounded-full bg-amber-500"></div>
              </div>
            </div>
          </div>
        </div>

        <div class="rounded-2xl border border-slate-800 bg-slate-900 p-6">
          <h2 class="text-xl font-semibold">Security layer</h2>
          <ul class="mt-5 space-y-3 text-sm text-slate-300">
            <li>• JWT authentication</li>
            <li>• Rate limiting by IP and user</li>
            <li>• Argon2 password hashing</li>
            <li>• Encrypted secret storage patterns</li>
            <li>• CORS + security headers</li>
            <li>• Audit logging hooks</li>
          </ul>
        </div>
      </section>
    </main>
  </div>
</template>
