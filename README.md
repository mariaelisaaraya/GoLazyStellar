# 🏆 GoLazy — Reta, Participa y Gana con Stellar

> dApp en Soroban para crear y gestionar desafíos con recompensas automáticas en XLM.

---

## 📌 Descripción General

**GoLazy** es una aplicación basada en Soroban que permite crear desafíos comunitarios con recompensas económicas usando **XLM**.  
Su objetivo es fomentar la participación colectiva, la transparencia y la inclusión financiera mediante incentivos programados y distribución automática de fondos.

---

## 💡 Problema que Resuelve

Organizar retos o convocatorias con premios suele ser complejo, poco transparente o centralizado.  
**GoLazy** transforma ese proceso en algo simple, confiable y sin intermediarios:

- Proyectos comunitarios.
- Desafíos educativos.
- Incentivos por objetivos cumplidos.
- Participación abierta en dinámicas colaborativas.

La recompensa en XLM se mantiene en **escrow** hasta que se definen los ganadores, lo que garantiza seguridad y confianza para todos los participantes.

---

## ✨ Funcionalidades Principales

- 🛠️ Crear desafíos con título, descripción, recompensa y fecha límite.
- 🧑‍🤝‍🧑 Participación abierta con verificación automática de elegibilidad.
- 🏅 Distribución automática de XLM entre los ganadores seleccionados.
- 🔒 Escrow nativo: fondos depositados se liberan sólo al final.
- 🔁 Devolución del sobrante al creador si quedan XLM sin repartir.
- 📦 Código 100% open source y documentado.


GoLazy utiliza el **stack de Stellar** de forma nativa:

- ✅ **Soroban** (contrato inteligente en Rust)
- ✅ **XLM como token nativo**
- ✅ **Gestión de identidades con `require_auth()`**
- ✅ **Transferencias automáticas vía `token::Client`**

