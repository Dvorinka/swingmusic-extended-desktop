<template>
  <div v-if="isVisible" class="web-app-connection-modal" @click="closeModal">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>Connect to Web App</h2>
        <button @click="closeModal" class="close-btn">
          <X />
        </button>
      </div>
      
      <div class="modal-body">
        <div class="connection-status" :class="{ connected: isConnected }">
          <div class="status-icon">
            <Cloud v-if="!isConnected" />
            <CheckCircle v-else />
          </div>
          <div class="status-text">
            <h3>{{ isConnected ? 'Connected' : 'Not Connected' }}</h3>
            <p>{{ isConnected ? `Connected to ${webAppUrl}` : 'Connect to sync your library and access web features' }}</p>
          </div>
        </div>
        
        <div v-if="!isConnected" class="connection-form">
          <div class="form-group">
            <label>Web App URL</label>
            <input 
              v-model="localWebAppUrl" 
              type="url" 
              placeholder="http://localhost:1970"
              class="url-input"
            />
          </div>

          <div class="auth-mode">
            <button
              class="mode-btn"
              :class="{ active: authMode === 'pair' }"
              @click="authMode = 'pair'"
            >
              Pair Code
            </button>
            <button
              class="mode-btn"
              :class="{ active: authMode === 'manual' }"
              @click="authMode = 'manual'"
            >
              Username & Password
            </button>
          </div>
          
          <div class="pairing-section" v-if="authMode === 'pair'">
            <h4>Pairing Code</h4>
            <input
              v-model="pairingCode"
              type="text"
              placeholder="Enter code from web UI pairing screen"
              class="url-input"
              autocomplete="off"
              autocapitalize="characters"
            />
            <p class="code-instruction">
              Generate this code in the web app settings and paste it here.
            </p>
          </div>

          <div class="pairing-section" v-else>
            <h4>Account Login</h4>
            <input
              v-model="username"
              type="text"
              placeholder="Username"
              class="url-input"
              autocomplete="username"
            />
            <input
              v-model="password"
              type="password"
              placeholder="Password"
              class="url-input"
              autocomplete="current-password"
            />
            <p class="code-instruction">
              Use this when QR pairing is not available.
            </p>
          </div>
          
          <div class="connection-benefits">
            <h4>Why connect?</h4>
            <ul>
              <li><CheckCircle /> Access your entire music library</li>
              <li><CheckCircle /> Sync playlists and favorites</li>
              <li><CheckCircle /> Stream from web app to desktop</li>
              <li><CheckCircle /> Control playback from any device</li>
              <li><CheckCircle /> Download tracks for offline listening</li>
            </ul>
          </div>
          <p v-if="errorMessage" class="error-text">{{ errorMessage }}</p>
        </div>
        
        <div v-else class="connected-actions">
          <div class="sync-status">
            <h4>Sync Status</h4>
            <div class="sync-items">
              <div v-for="item in syncItems" :key="item" class="sync-item">
                <CheckCircle class="sync-icon" />
                <span>{{ item }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button
          v-if="!isConnected"
          @click="connect"
          class="connect-btn primary"
          :disabled="!canConnect || isConnecting"
        >
          <Loader v-if="isConnecting" />
          <Cloud v-else />
          {{ isConnecting ? 'Connecting...' : 'Connect' }}
        </button>
        <button v-else @click="disconnect" class="disconnect-btn secondary">
          <Unlink /> Disconnect
        </button>
        <button @click="closeModal" class="cancel-btn">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface Props {
  isVisible: boolean
  webAppUrl: string
  isConnected: boolean
}

interface Emits {
  (e: 'close'): void
  (e: 'connection-changed', connected: boolean): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localWebAppUrl = ref(props.webAppUrl)
const pairingCode = ref('')
const username = ref('')
const password = ref('')
const authMode = ref<'pair' | 'manual'>('pair')
const isConnecting = ref(false)
const syncItems = ref<string[]>([])
const errorMessage = ref('')

const canConnect = computed(() => {
  if (!localWebAppUrl.value.trim()) return false
  if (authMode.value === 'pair') {
    return pairingCode.value.trim().length > 0
  }
  return username.value.trim().length > 0 && password.value.length > 0
})

// Watch for prop changes
watch(() => props.webAppUrl, (newUrl) => {
  localWebAppUrl.value = newUrl
})

watch(() => props.isConnected, (connected) => {
  if (connected) {
    loadSyncStatus()
  }
})

const connect = async () => {
  if (!localWebAppUrl.value.trim()) return
  if (authMode.value === 'pair' && !pairingCode.value.trim()) return
  if (authMode.value === 'manual' && (!username.value.trim() || !password.value)) return
  
  isConnecting.value = true
  errorMessage.value = ''
  try {
    const connected = await invoke<boolean>('connect_to_web_app', {
      request: {
        url: localWebAppUrl.value.trim(),
        pairingCode: authMode.value === 'pair' ? pairingCode.value.trim().toUpperCase() : '',
        username: authMode.value === 'manual' ? username.value.trim() : null,
        password: authMode.value === 'manual' ? password.value : null,
      },
    })
    
    if (connected) {
      emit('connection-changed', true)
      await loadSyncStatus()
    }
  } catch (error) {
    errorMessage.value = String(error)
  } finally {
    isConnecting.value = false
  }
}

const disconnect = async () => {
  try {
    await invoke('disconnect_from_web_app')
    emit('connection-changed', false)
    pairingCode.value = ''
    username.value = ''
    password.value = ''
    errorMessage.value = ''
  } catch (error) {
    console.error('Disconnect failed:', error)
  }
}

const loadSyncStatus = async () => {
  try {
    syncItems.value = await invoke('sync_with_web_app')
  } catch (error) {
    console.error('Failed to load sync status:', error)
  }
}

const closeModal = () => {
  emit('close')
}
</script>

<style scoped>
.web-app-connection-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--surface);
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border);
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  color: var(--on-surface-variant);
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--surface-variant);
}

.modal-body {
  padding: 20px;
  max-height: 60vh;
  overflow-y: auto;
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  border-radius: 8px;
  background: var(--surface-variant);
  margin-bottom: 20px;
}

.connection-status.connected {
  background: var(--primary-container);
  color: var(--on-primary-container);
}

.status-icon {
  font-size: 2rem;
}

.status-text h3 {
  margin: 0 0 4px 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.status-text p {
  margin: 0;
  opacity: 0.8;
}

.form-group {
  margin-bottom: 20px;
}

.auth-mode {
  display: flex;
  gap: 8px;
  margin-bottom: 14px;
}

.mode-btn {
  border: 1px solid var(--border);
  background: transparent;
  color: var(--on-surface-variant);
  border-radius: 8px;
  padding: 8px 12px;
  cursor: pointer;
}

.mode-btn.active {
  border-color: var(--primary);
  color: var(--primary);
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
}

.url-input {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--surface);
  color: var(--on-surface);
  font-size: 1rem;
}

.pairing-section .url-input + .url-input {
  margin-top: 8px;
}

.pairing-section {
  margin-bottom: 20px;
}

.pairing-section h4 {
  margin: 0 0 12px 0;
  font-size: 1rem;
  font-weight: 600;
}

.pairing-code-display {
  text-align: center;
  margin-bottom: 12px;
}

.code-box {
  display: inline-block;
  padding: 16px 24px;
  background: var(--primary-container);
  color: var(--on-primary-container);
  font-size: 1.5rem;
  font-weight: bold;
  letter-spacing: 2px;
  border-radius: 8px;
  margin-bottom: 8px;
}

.code-instruction {
  margin: 0;
  font-size: 0.9rem;
  opacity: 0.7;
}

.error-text {
  margin-top: 12px;
  color: #f7635c;
  font-size: 0.9rem;
}

.generate-code-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--surface-variant);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.generate-code-btn:hover {
  background: var(--surface);
}

.connection-benefits h4 {
  margin: 0 0 12px 0;
  font-size: 1rem;
  font-weight: 600;
}

.connection-benefits ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.connection-benefits li {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  font-size: 0.9rem;
}

.connection-benefits li svg {
  color: var(--primary);
  flex-shrink: 0;
}

.sync-items {
  margin-top: 12px;
}

.sync-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
}

.sync-icon {
  color: var(--primary);
}

.modal-footer {
  display: flex;
  gap: 12px;
  padding: 20px;
  border-top: 1px solid var(--border);
  justify-content: flex-end;
}

.connect-btn, .disconnect-btn, .cancel-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.connect-btn {
  background: var(--primary);
  color: var(--on-primary);
}

.connect-btn:hover:not(:disabled) {
  background: var(--primary-variant);
}

.connect-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.disconnect-btn {
  background: var(--error);
  color: var(--on-error);
}

.disconnect-btn:hover {
  background: var(--error-variant);
}

.cancel-btn {
  background: var(--surface-variant);
  color: var(--on-surface-variant);
}

.cancel-btn:hover {
  background: var(--surface);
}
</style>
