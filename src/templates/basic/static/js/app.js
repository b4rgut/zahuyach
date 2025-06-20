/**
 * Zahuyach Blog - Main JavaScript File
 * GitHub Dark Theme Blog Generator
 */

'use strict';

// Global app state
window.ZahuyachApp = {
    version: '1.0.0',
    initialized: false,
    settings: {
        theme: 'dark',
        viewMode: 'list', // 'list' or 'grid'
        sidebarCollapsed: false,
        searchModalOpen: false
    },
    cache: new Map(),
    utils: {},
    ui: {},
    search: {},
    bookmarks: {},
    categories: {}
};

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    ZahuyachApp.init();
});

/**
 * Main initialization function
 */
ZahuyachApp.init = function() {
    if (this.initialized) return;

    console.log(`🚀 Zahuyach Blog v${this.version} - Initializing...`);

    // Load settings from localStorage
    this.loadSettings();

    // Initialize components
    this.ui.init();
    this.search.init();
    this.bookmarks.init();
    this.categories.init();
    this.utils.init();

    // Setup HTMX integration
    this.setupHTMX();

    // Setup keyboard shortcuts
    this.setupKeyboardShortcuts();

    // Setup service worker (if available)
    this.setupServiceWorker();

    this.initialized = true;
    console.log('✅ Zahuyach Blog initialized successfully');

    // Dispatch custom event
    document.dispatchEvent(new CustomEvent('zahuyach:initialized'));
};

/**
 * Load settings from localStorage
 */
ZahuyachApp.loadSettings = function() {
    try {
        const saved = localStorage.getItem('zahuyach-settings');
        if (saved) {
            this.settings = { ...this.settings, ...JSON.parse(saved) };
        }
    } catch (error) {
        console.warn('Failed to load settings:', error);
    }
};

/**
 * Save settings to localStorage
 */
ZahuyachApp.saveSettings = function() {
    try {
        localStorage.setItem('zahuyach-settings', JSON.stringify(this.settings));
    } catch (error) {
        console.warn('Failed to save settings:', error);
    }
};

/**
 * UI Management
 */
ZahuyachApp.ui = {
    // Initialize UI components
    init() {
        this.setupMobileMenu();
        this.setupSearchModal();
        this.setupSidebar();
        this.setupScrollHandlers();
        this.setupTooltips();
        this.restoreViewMode();

        console.log('🎨 UI components initialized');
    },

    // Mobile menu functionality
    setupMobileMenu() {
        const toggleBtn = document.getElementById('mobile-menu-toggle');
        const menu = document.getElementById('mobile-menu');

        if (toggleBtn && menu) {
            toggleBtn.addEventListener('click', () => {
                menu.classList.toggle('hidden');
                toggleBtn.setAttribute('aria-expanded',
                    menu.classList.contains('hidden') ? 'false' : 'true'
                );
            });
        }
    },

    // Search modal functionality
    setupSearchModal() {
        const searchToggle = document.getElementById('search-toggle');
        const searchOverlay = document.getElementById('search-overlay');
        const searchBackdrop = document.getElementById('search-backdrop');
        const closeSearch = document.getElementById('close-search');

        if (searchToggle && searchOverlay) {
            // Open search modal
            searchToggle.addEventListener('click', (e) => {
                e.preventDefault();
                this.openSearchModal();
            });

            // Close search modal
            [searchBackdrop, closeSearch].forEach(element => {
                if (element) {
                    element.addEventListener('click', (e) => {
                        e.preventDefault();
                        this.closeSearchModal();
                    });
                }
            });
        }
    },

    // Open search modal
    openSearchModal() {
        const searchOverlay = document.getElementById('search-overlay');
        const searchInput = document.getElementById('modal-search-input');

        if (searchOverlay) {
            searchOverlay.classList.remove('hidden');
            ZahuyachApp.settings.searchModalOpen = true;

            // Focus search input after animation
            setTimeout(() => {
                if (searchInput) {
                    searchInput.focus();
                }
            }, 100);

            // Prevent body scroll
            document.body.style.overflow = 'hidden';
        }
    },

    // Close search modal
    closeSearchModal() {
        const searchOverlay = document.getElementById('search-overlay');

        if (searchOverlay) {
            searchOverlay.classList.add('hidden');
            ZahuyachApp.settings.searchModalOpen = false;

            // Restore body scroll
            document.body.style.overflow = '';
        }
    },

    // Sidebar functionality
    setupSidebar() {
        const sidebarToggle = document.getElementById('sidebar-toggle');
        const mobileSidebarOverlay = document.getElementById('mobile-sidebar-overlay');

        if (sidebarToggle) {
            sidebarToggle.addEventListener('click', () => {
                this.toggleMobileSidebar();
            });
        }

        // Close sidebar when clicking overlay
        if (mobileSidebarOverlay) {
            mobileSidebarOverlay.addEventListener('click', (e) => {
                if (e.target === mobileSidebarOverlay) {
                    this.closeMobileSidebar();
                }
            });
        }
    },

    // Toggle mobile sidebar
    toggleMobileSidebar() {
        const overlay = document.getElementById('mobile-sidebar-overlay');
        const sidebar = document.getElementById('mobile-sidebar');

        if (overlay && sidebar) {
            const isHidden = overlay.classList.contains('hidden');

            if (isHidden) {
                overlay.classList.remove('hidden');
                setTimeout(() => {
                    sidebar.classList.remove('translate-x-full');
                }, 10);
                document.body.style.overflow = 'hidden';
            } else {
                this.closeMobileSidebar();
            }
        }
    },

    // Close mobile sidebar
    closeMobileSidebar() {
        const overlay = document.getElementById('mobile-sidebar-overlay');
        const sidebar = document.getElementById('mobile-sidebar');

        if (overlay && sidebar) {
            sidebar.classList.add('translate-x-full');
            setTimeout(() => {
                overlay.classList.add('hidden');
                document.body.style.overflow = '';
            }, 300);
        }
    },

    // Toggle view mode (list/grid)
    toggleViewMode(mode) {
        const container = document.getElementById('posts-container');
        const gridBtn = document.getElementById('grid-view');
        const listBtn = document.getElementById('list-view');

        if (container && gridBtn && listBtn) {
            ZahuyachApp.settings.viewMode = mode;

            // Update container class
            if (mode === 'grid') {
                container.className = 'posts-grid-view';
                gridBtn.classList.add('bg-gh-canvas-subtle', 'text-gh-fg-default');
                gridBtn.classList.remove('text-gh-fg-muted');
                listBtn.classList.remove('bg-gh-canvas-subtle', 'text-gh-fg-default');
                listBtn.classList.add('text-gh-fg-muted');
            } else {
                container.className = 'posts-list-view';
                listBtn.classList.add('bg-gh-canvas-subtle', 'text-gh-fg-default');
                listBtn.classList.remove('text-gh-fg-muted');
                gridBtn.classList.remove('bg-gh-canvas-subtle', 'text-gh-fg-default');
                gridBtn.classList.add('text-gh-fg-muted');
            }

            ZahuyachApp.saveSettings();
        }
    },

    // Restore view mode from settings
    restoreViewMode() {
        this.toggleViewMode(ZahuyachApp.settings.viewMode);
    },

    // Setup scroll handlers
    setupScrollHandlers() {
        let scrollTimeout;
        const header = document.querySelector('header');

        window.addEventListener('scroll', () => {
            clearTimeout(scrollTimeout);

            if (header) {
                if (window.scrollY > 100) {
                    header.classList.add('scrolled');
                } else {
                    header.classList.remove('scrolled');
                }
            }

            scrollTimeout = setTimeout(() => {
                // Scroll ended
            }, 150);
        });
    },

    // Setup tooltips
    setupTooltips() {
        document.addEventListener('mouseenter', (e) => {
            const element = e.target.closest('[title]');
            if (element && element.title) {
                this.showTooltip(element, element.title);
            }
        }, true);

        document.addEventListener('mouseleave', (e) => {
            const element = e.target.closest('[title]');
            if (element) {
                this.hideTooltip();
            }
        }, true);
    },

    // Show tooltip
    showTooltip(element, text) {
        // Simple tooltip implementation
        const existing = document.getElementById('app-tooltip');
        if (existing) existing.remove();

        const tooltip = document.createElement('div');
        tooltip.id = 'app-tooltip';
        tooltip.className = 'fixed z-50 px-2 py-1 text-xs bg-gh-canvas-overlay border border-gh-border-default rounded shadow-lg pointer-events-none';
        tooltip.textContent = text;

        document.body.appendChild(tooltip);

        const rect = element.getBoundingClientRect();
        tooltip.style.left = `${rect.left + rect.width / 2 - tooltip.offsetWidth / 2}px`;
        tooltip.style.top = `${rect.top - tooltip.offsetHeight - 8}px`;
    },

    // Hide tooltip
    hideTooltip() {
        const tooltip = document.getElementById('app-tooltip');
        if (tooltip) {
            tooltip.remove();
        }
    },

    // Show notification
    showNotification(message, type = 'info', duration = 3000) {
        const notification = document.createElement('div');
        notification.className = `fixed top-4 right-4 z-50 bg-gh-canvas-overlay border border-gh-border-default rounded-lg px-4 py-3 shadow-lg transform translate-x-full transition-transform duration-300 max-w-sm`;

        const icons = {
            info: `<svg class="w-5 h-5 text-gh-accent-fg" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>`,
            success: `<svg class="w-5 h-5 text-gh-success-fg" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>`,
            warning: `<svg class="w-5 h-5 text-gh-attention-fg" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z"/></svg>`,
            error: `<svg class="w-5 h-5 text-gh-danger-fg" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>`
        };

        notification.innerHTML = `
            <div class="flex items-start space-x-3">
                ${icons[type] || icons.info}
                <div class="flex-1 text-sm text-gh-fg-default">${message}</div>
                <button class="text-gh-fg-muted hover:text-gh-fg-default" onclick="this.parentElement.parentElement.remove()">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                </button>
            </div>
        `;

        document.body.appendChild(notification);

        // Animate in
        setTimeout(() => notification.classList.remove('translate-x-full'), 10);

        // Auto remove
        if (duration > 0) {
            setTimeout(() => {
                notification.classList.add('translate-x-full');
                setTimeout(() => notification.remove(), 300);
            }, duration);
        }
    }
};

/**
 * Search functionality
 */
ZahuyachApp.search = {
    init() {
        this.setupSearchInput();
        this.loadRecentSearches();
        console.log('🔍 Search functionality initialized');
    },

    setupSearchInput() {
        const searchInput = document.getElementById('search-input');
        const modalSearchInput = document.getElementById('modal-search-input');

        [searchInput, modalSearchInput].forEach(input => {
            if (input) {
                input.addEventListener('input', (e) => {
                    this.handleSearchInput(e.target.value);
                });
            }
        });
    },

    handleSearchInput(query) {
        if (query.length >= 2) {
            this.performSearch(query);
        }
    },

    performSearch(query) {
        // Save to recent searches
        this.saveRecentSearch(query);
    },

    loadRecentSearches() {
        try {
            const recent = localStorage.getItem('zahuyach-recent-searches');
            return recent ? JSON.parse(recent) : [];
        } catch (error) {
            console.warn('Failed to load recent searches:', error);
            return [];
        }
    },

    saveRecentSearch(query) {
        if (!query.trim()) return;

        try {
            let recent = this.loadRecentSearches();
            recent = recent.filter(item => item !== query);
            recent.unshift(query);
            recent = recent.slice(0, 10); // Keep only 10 recent searches

            localStorage.setItem('zahuyach-recent-searches', JSON.stringify(recent));
        } catch (error) {
            console.warn('Failed to save recent search:', error);
        }
    }
};

/**
 * Bookmarks functionality
 */
ZahuyachApp.bookmarks = {
    init() {
        this.updateBookmarkButtons();
        this.updateBookmarkCount();
        console.log('🔖 Bookmarks functionality initialized');
    },

    getBookmarks() {
        try {
            const bookmarks = localStorage.getItem('zahuyach-bookmarks');
            return bookmarks ? JSON.parse(bookmarks) : [];
        } catch (error) {
            console.warn('Failed to load bookmarks:', error);
            return [];
        }
    },

    saveBookmarks(bookmarks) {
        try {
            localStorage.setItem('zahuyach-bookmarks', JSON.stringify(bookmarks));
            this.updateBookmarkCount();

            // Dispatch custom event
            document.dispatchEvent(new CustomEvent('zahuyach:bookmarksChanged', {
                detail: { bookmarks }
            }));
        } catch (error) {
            console.warn('Failed to save bookmarks:', error);
        }
    },

    toggleBookmark(slug) {
        const bookmarks = this.getBookmarks();
        const index = bookmarks.indexOf(slug);

        if (index > -1) {
            bookmarks.splice(index, 1);
            ZahuyachApp.ui.showNotification('Статья удалена из закладок', 'info');
        } else {
            bookmarks.push(slug);
            ZahuyachApp.ui.showNotification('Статья добавлена в закладки', 'success');
        }

        this.saveBookmarks(bookmarks);
        this.updateBookmarkButtons();
    },

    updateBookmarkButtons() {
        const bookmarks = this.getBookmarks();
        const buttons = document.querySelectorAll('[data-post-slug]');

        buttons.forEach(button => {
            const slug = button.dataset.postSlug;
            const isBookmarked = bookmarks.includes(slug);

            if (isBookmarked) {
                button.classList.add('text-gh-accent-fg');
                button.classList.remove('text-gh-fg-muted');
            } else {
                button.classList.remove('text-gh-accent-fg');
                button.classList.add('text-gh-fg-muted');
            }
        });
    },

    updateBookmarkCount() {
        const bookmarks = this.getBookmarks();
        const countElement = document.getElementById('bookmarks-count');

        if (countElement) {
            countElement.textContent = bookmarks.length;
            countElement.style.display = bookmarks.length > 0 ? 'inline' : 'none';
        }
    }
};

/**
 * Categories functionality
 */
ZahuyachApp.categories = {
    init() {
        this.loadExpandedCategories();
        console.log('📁 Categories functionality initialized');
    },

    getExpandedCategories() {
        try {
            const expanded = localStorage.getItem('zahuyach-expanded-categories');
            return expanded ? JSON.parse(expanded) : [];
        } catch (error) {
            console.warn('Failed to load expanded categories:', error);
            return [];
        }
    },

    saveExpandedCategories(categories) {
        try {
            localStorage.setItem('zahuyach-expanded-categories', JSON.stringify(categories));
        } catch (error) {
            console.warn('Failed to save expanded categories:', error);
        }
    },

    toggleCategory(slug) {
        const expanded = this.getExpandedCategories();
        const index = expanded.indexOf(slug);

        if (index > -1) {
            expanded.splice(index, 1);
        } else {
            expanded.push(slug);
        }

        this.saveExpandedCategories(expanded);
    },

    loadExpandedCategories() {
        const expanded = this.getExpandedCategories();

        expanded.forEach(slug => {
            const element = document.querySelector(`[data-category-children="${slug}"]`);
            const chevron = document.querySelector(`[data-category="${slug}"] .category-chevron`);

            if (element) {
                element.classList.remove('hidden');
            }
            if (chevron) {
                chevron.classList.add('rotate-90');
            }
        });
    }
};

/**
 * Utilities
 */
ZahuyachApp.utils = {
    init() {
        this.setupShareButtons();
        this.setupCopyButtons();
        console.log('🛠️ Utilities initialized');
    },

    setupShareButtons() {
        document.addEventListener('click', (e) => {
            if (e.target.matches('[data-share]') || e.target.closest('[data-share]')) {
                e.preventDefault();
                const button = e.target.matches('[data-share]') ? e.target : e.target.closest('[data-share]');
                const title = button.dataset.title || document.title;
                const url = button.dataset.url || window.location.href;

                this.sharePost(title, url);
            }
        });
    },

    setupCopyButtons() {
        document.addEventListener('click', (e) => {
            if (e.target.matches('[data-copy]') || e.target.closest('[data-copy]')) {
                e.preventDefault();
                const button = e.target.matches('[data-copy]') ? e.target : e.target.closest('[data-copy]');
                const text = button.dataset.copy;

                this.copyToClipboard(text);
            }
        });
    },

    async sharePost(title, url) {
        if (navigator.share) {
            try {
                await navigator.share({ title, url });
            } catch (error) {
                if (error.name !== 'AbortError') {
                    this.copyToClipboard(url);
                }
            }
        } else {
            this.copyToClipboard(url);
        }
    },

    async copyToClipboard(text) {
        try {
            await navigator.clipboard.writeText(text);
            ZahuyachApp.ui.showNotification('Скопировано в буфер обмена', 'success');
        } catch (error) {
            // Fallback for older browsers
            const textArea = document.createElement('textarea');
            textArea.value = text;
            textArea.style.position = 'fixed';
            textArea.style.opacity = '0';
            document.body.appendChild(textArea);
            textArea.focus();
            textArea.select();

            try {
                document.execCommand('copy');
                ZahuyachApp.ui.showNotification('Скопировано в буфер обмена', 'success');
            } catch (fallbackError) {
                ZahuyachApp.ui.showNotification('Не удалось скопировать', 'error');
            }

            document.body.removeChild(textArea);
        }
    },

    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    },

    throttle(func, limit) {
        let inThrottle;
        return function() {
            const args = arguments;
            const context = this;
            if (!inThrottle) {
                func.apply(context, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    },

    formatDate(date, locale = 'ru-RU') {
        return new Intl.DateTimeFormat(locale, {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        }).format(new Date(date));
    },

    formatRelativeTime(date) {
        const now = new Date();
        const diff = now - new Date(date);
        const seconds = Math.floor(diff / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);
        const days = Math.floor(hours / 24);

        if (days > 0) return `${days} дн. назад`;
        if (hours > 0) return `${hours} ч. назад`;
        if (minutes > 0) return `${minutes} мин. назад`;
        return 'только что';
    }
};

/**
 * HTMX Integration
 */
ZahuyachApp.setupHTMX = function() {
    // HTMX event handlers
    document.addEventListener('htmx:beforeRequest', (e) => {
        // Show loading indicator
        const indicator = document.getElementById('htmx-indicator');
        if (indicator) {
            indicator.style.display = 'flex';
        }
    });

    document.addEventListener('htmx:afterRequest', (e) => {
        // Hide loading indicator
        const indicator = document.getElementById('htmx-indicator');
        if (indicator) {
            indicator.style.display = 'none';
        }

        // Re-initialize components for new content
        if (e.detail.target.id === 'posts-container') {
            this.bookmarks.updateBookmarkButtons();
        }
    });

    document.addEventListener('htmx:responseError', (e) => {
        this.ui.showNotification('Произошла ошибка при загрузке', 'error');
        console.error('HTMX Error:', e.detail);
    });

    document.addEventListener('htmx:timeout', (e) => {
        this.ui.showNotification('Превышено время ожидания', 'warning');
    });

    console.log('🔄 HTMX integration setup complete');
};

/**
 * Keyboard shortcuts
 */
ZahuyachApp.setupKeyboardShortcuts = function() {
    document.addEventListener('keydown', (e) => {
        // Ignore if typing in input fields
        if (e.target.matches('input, textarea, select')) return;

        // Handle shortcuts
        switch (e.key) {
            case '/':
                e.preventDefault();
                this.ui.openSearchModal();
                break;

            case 'Escape':
                if (this.settings.searchModalOpen) {
                    this.ui.closeSearchModal();
                }
                break;

            case 'g':
                if (e.ctrlKey || e.metaKey) {
                    e.preventDefault();
                    this.ui.toggleViewMode(
                        this.settings.viewMode === 'list' ? 'grid' : 'list'
                    );
                }
                break;
        }
    });

    console.log('⌨️ Keyboard shortcuts setup complete');
};

/**
 * Service Worker setup
 */
ZahuyachApp.setupServiceWorker = function() {
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker.register('/sw.js')
            .then(registration => {
                console.log('🔧 Service Worker registered successfully');
            })
            .catch(error => {
                console.log('Service Worker registration failed:', error);
            });
    }
};

// Global functions for template usage
window.toggleView = function(mode) {
    ZahuyachApp.ui.toggleViewMode(mode);
};

window.toggleSidebar = function() {
    ZahuyachApp.ui.toggleMobileSidebar();
};

window.toggleBookmark = function(slug) {
    ZahuyachApp.bookmarks.toggleBookmark(slug);
};

window.toggleCategory = function(slug) {
    ZahuyachApp.categories.toggleCategory(slug);
};

window.sharePost = function(title, url) {
    ZahuyachApp.utils.sharePost(title, url);
};

window.showNotification = function(message, type, duration) {
    ZahuyachApp.ui.showNotification(message, type, duration);
};

// Export for ES modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ZahuyachApp;
}
