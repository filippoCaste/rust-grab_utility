# Panoramica
## Crates
- **screenshots**: https://lib.rs/crates/screenshots
- **egui**: https://docs.rs/egui/latest/egui/ (github: https://github.com/emilk/egui.git)
- **native-dialog**: https://docs.rs/native-dialog/latest/native_dialog/

## Funzionalità
1. [x] **Platform Support**: l'app può essere utilizzata su tutte le piattaforme.
2. [x] **User Interface (UI)**: interfaccia utente facilmente usufruibile.
3. [x] **Selection Options**: Riquadro di selezione della schermata per ritagliare la porzione di schermo da catturare.
4. [ ] **Hotkey Support**: The utility should support customizable hotkeys for quick
screen grabbing. Users should be able to set up their preferred shortcut keys.
5. [x] **Output Format**: diversi formati supportati per il salvataggio della schermata (png, jpg, gif). Salvataggio anche negli appunti del dispositivo.
    - [ ] manca salvare negli appunti del dispositivo
<!-- FUNZIONALITÀ BONUS -->
6. [ ] **Annotation Tools**: The utility should have built-in annotation tools like
shapes, arrows, text, and a color picker for highlighting or redacting parts of
the screen grab.
7. [x] **Delay Timer**: tramite l'interfaccia, l'utente può impostare il delay dopo il quale la schermata sarà catturata.
8. [x] **Save Options**: viene aperta una finestra di dialogo del file system attraverso la quale l'utente può scegliere il nome (se diverso da default) e il formato dell'immagine.
9.  [x] **Multi-monitor Support**: possibilità di catturare le schermate anche degli altri schermi.

## Esempio di utilizzo
![Screenshot d'esempio](esempio.png)
- `🖵`: per catturare la schermata intera
- `⛶`: per catturare soltanto la porzione di schermo inquadrata
- `🕓`: per impostare un timer
- `Options`: per aprire il riquadro dove modificare le opzioni
- `X`: per chiudere l'applicazione

