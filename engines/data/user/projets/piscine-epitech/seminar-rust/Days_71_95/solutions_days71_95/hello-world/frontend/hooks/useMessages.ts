"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import { listMessages, updateMessage as apiUpdateMessage, deleteMessage as apiDeleteMessage, Message } from "@/lib/api-server";
import { handleAuthError, isAuthError, getErrorMessage } from "@/lib/auth/utils";
import { useWebSocket } from "./useWebSocket";
import { ServerEvent } from "@/lib/gateway";

/**
 * Hook pour gérer les messages d'un channel
 * Utilise WebSocket pour les nouveaux messages en temps réel
 * et REST pour charger l'historique initial
 */
export function useMessages(channelId: string | null) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(false);
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [typingUsers, setTypingUsers] = useState<Map<string, string>>(new Map()); // user_id -> username
  const { onEvent, subscribe, unsubscribe, sendMessage: wsSendMessage, typingStart, typingStop, isConnected } = useWebSocket();
  const subscribedRef = useRef(false);
  const typingTimeoutRef = useRef<Map<string, NodeJS.Timeout>>(new Map());

  // Charger l'historique initial via REST
  const loadMessages = useCallback(async (id: string, showLoading = false) => {
    if (!id) {
      setMessages([]);
      return;
    }
    
    try {
      if (showLoading) setLoading(true);
      setError(null);
      const data = await listMessages(id);
      const messagesArray = Array.isArray(data) ? data : [];
      setMessages(messagesArray);
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to load messages");
      setError(errorMessage);
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return;
      }
      setMessages([]);
    } finally {
      if (showLoading) setLoading(false);
    }
  }, []);

  // Charger l'historique quand le channel change
  useEffect(() => {
    if (channelId) {
      loadMessages(channelId, true);
      subscribedRef.current = false;
    } else {
      setMessages([]);
    }
  }, [channelId, loadMessages]);

  // S'abonner au channel via WebSocket quand connecté
  useEffect(() => {
    if (!channelId || !isConnected()) return;

    // S'abonner au channel
    subscribe(channelId);
    subscribedRef.current = true;

    return () => {
      unsubscribe(channelId);
      subscribedRef.current = false;
    };
  }, [channelId, subscribe, unsubscribe, isConnected]);

  // Écouter les événements WebSocket
  useEffect(() => {
    if (!channelId) return;

    const unsubscribe = onEvent((event: ServerEvent) => {
      switch (event.op) {
        case "MESSAGE_CREATE":
          // Nouveau message reçu
          if (event.d.channel_id === channelId) {
            setMessages((prev) => {
              // Éviter les doublons
              if (prev.some((m) => m.id === event.d.id)) {
                return prev;
              }
              return [...prev, event.d];
            });
          }
          break;

        case "MESSAGE_UPDATE":
          // Message modifié
          if (event.d.channel_id === channelId) {
            setMessages((prev) =>
              prev.map((m) =>
                m.id === event.d.id
                  ? { ...m, content: event.d.content, edited_at: event.d.edited_at }
                  : m
              )
            );
          }
          break;

        case "MESSAGE_DELETE":
          // Message supprimé
          if (event.d.channel_id === channelId) {
            setMessages((prev) => prev.filter((m) => m.id !== event.d.id));
          }
          break;

        case "TYPING_START":
          // Quelqu'un commence à taper
          if (event.d.channel_id === channelId) {
            setTypingUsers((prev) => {
              const next = new Map(prev);
              next.set(event.d.user_id, event.d.username);
              return next;
            });
            
            // Nettoyer le timeout existant si présent
            const existingTimeout = typingTimeoutRef.current.get(event.d.user_id);
            if (existingTimeout) {
              clearTimeout(existingTimeout);
            }
            
            // Timeout automatique après 3 secondes
            const timeout = setTimeout(() => {
              setTypingUsers((prev) => {
                const next = new Map(prev);
                next.delete(event.d.user_id);
                return next;
              });
              typingTimeoutRef.current.delete(event.d.user_id);
            }, 3000);
            
            typingTimeoutRef.current.set(event.d.user_id, timeout);
          }
          break;

        case "TYPING_STOP":
          // Quelqu'un arrête de taper
          if (event.d.channel_id === channelId) {
            setTypingUsers((prev) => {
              const next = new Map(prev);
              next.delete(event.d.user_id);
              return next;
            });
            
            // Nettoyer le timeout
            const timeout = typingTimeoutRef.current.get(event.d.user_id);
            if (timeout) {
              clearTimeout(timeout);
              typingTimeoutRef.current.delete(event.d.user_id);
            }
          }
          break;

        case "ERROR":
          // Erreur serveur
          if (event.d.code === "MESSAGE_ERROR") {
            setError(event.d.message);
      }
          break;
      }
    });

    return unsubscribe;
  }, [channelId, onEvent]);

  // Envoyer un message via WebSocket
  const sendMessage = useCallback(async (content: string): Promise<boolean> => {
    if (!channelId || !content.trim()) return false;

    // Vérifier que WebSocket est connecté
    if (!isConnected()) {
      setError("Not connected to server. Please refresh the page.");
      return false;
    }

    try {
      setSending(true);
      setError(null);
      
      // Envoyer via WebSocket
      wsSendMessage(channelId, content.trim());
      
      // Le message sera ajouté automatiquement via MESSAGE_CREATE
      // Pas besoin de recharger
      return true;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to send message");
      setError(errorMessage);
      return false;
    } finally {
      setSending(false);
    }
  }, [channelId, wsSendMessage, isConnected]);

  // Éditer un message (via REST API, WebSocket broadcast automatique)
  const updateMessage = useCallback(async (id: string, content: string): Promise<boolean> => {
    if (!channelId || !content.trim()) return false;

    let previousMessages: Message[] = [];

    try {
      setError(null);
      
      // Optimisme UI : mise à jour immédiate
      setMessages((prev) => {
        previousMessages = [...prev];
        return prev.map((m) =>
          m.id === id
            ? { ...m, content: content.trim(), edited_at: new Date().toISOString() }
            : m
        );
      });

      await apiUpdateMessage(id, content.trim());
      
      // Le WebSocket MESSAGE_UPDATE va mettre à jour avec les données du serveur
      // Pas besoin de recharger manuellement
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousMessages.length > 0) {
        setMessages(previousMessages);
      }
      
      const errorMessage = getErrorMessage(err, "Failed to update message");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return false;
      }
      setError(errorMessage);
      return false;
    }
  }, [channelId]);

  // Supprimer un message (via REST API, WebSocket broadcast automatique)
  const deleteMessage = useCallback(async (id: string): Promise<boolean> => {
    if (!channelId) return false;

    let previousMessages: Message[] = [];

    try {
      setError(null);
      
      // Optimisme UI : suppression immédiate
      setMessages((prev) => {
        previousMessages = [...prev];
        return prev.filter((m) => m.id !== id);
      });

      await apiDeleteMessage(id);
      
      // Le WebSocket MESSAGE_DELETE va confirmer la suppression
      // Pas besoin de recharger manuellement
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousMessages.length > 0) {
        setMessages(previousMessages);
      }
      
      const errorMessage = getErrorMessage(err, "Failed to delete message");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return false;
      }
      setError(errorMessage);
      return false;
    }
  }, [channelId]);

  // Rafraîchir l'historique (utile pour resync après reconnexion)
  const refresh = useCallback(() => {
    if (channelId) {
      loadMessages(channelId, false);
    }
  }, [channelId, loadMessages]);

  return {
    messages,
    loading,
    sending,
    error,
    typingUsers,
    sendMessage,
    updateMessage,
    deleteMessage,
    refresh,
    isConnected: isConnected(),
    typingStart,
    typingStop,
  };
}

