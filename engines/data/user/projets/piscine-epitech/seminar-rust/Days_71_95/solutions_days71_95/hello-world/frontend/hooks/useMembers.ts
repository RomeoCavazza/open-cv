"use client";

import { useState, useEffect, useCallback } from "react";
import { banMember as apiBanMember, listMembers, kickMember as apiKickMember, ServerMember } from "@/lib/api-server";
import { handleAuthError, isAuthError, getErrorMessage } from "@/lib/auth/utils";
import { useWebSocket } from "./useWebSocket";
import { ServerEvent } from "@/lib/gateway";

export function useMembers(serverId: string | null) {
  const [members, setMembers] = useState<ServerMember[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  // État pour stocker les statuts des utilisateurs (user_id -> status)
  const [userStatuses, setUserStatuses] = useState<Map<string, string>>(new Map());
  const { onEvent } = useWebSocket();

  const loadMembers = useCallback(async (id: string) => {
    try {
      setLoading(true);
      setError(null);
      const data = await listMembers(id);
      setMembers(data);
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to load members");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return;
      }
      setError(errorMessage);
      setMembers([]);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (serverId) {
      loadMembers(serverId);
    } else {
      setMembers([]);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [serverId]);

  // Écouter les événements PRESENCE_UPDATE pour mettre à jour les statuts en temps réel
  useEffect(() => {
    const unsubscribe = onEvent((event: ServerEvent) => {
      if (event.op === "PRESENCE_UPDATE") {
        const { user_id, status } = event.d;
        setUserStatuses((prev) => {
          const next = new Map(prev);
          next.set(user_id, status);
          return next;
        });
      }
    });

    return unsubscribe;
  }, [onEvent]);

  const kickMember = useCallback(async (userId: string): Promise<boolean> => {
    if (!serverId) return false;
    // Optimistic UI : on retire le membre immédiatement
    setMembers((prev) => prev.filter((m) => m.user_id !== userId));
    try {
      await apiKickMember(serverId, userId);
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      loadMembers(serverId);
      const errorMessage = getErrorMessage(err, "Failed to kick member");
      if (isAuthError(errorMessage)) handleAuthError();
      return false;
    }
  }, [serverId, loadMembers]);

  const banMember = useCallback(async (userId: string): Promise<boolean> => {
    if (!serverId) return false;
    // Optimistic UI : on retire le membre immédiatement (ban implique kick)
    setMembers((prev) => prev.filter((m) => m.user_id !== userId));
    try {
      await apiBanMember(serverId, userId, {});
      return true;
    } catch (err) {
      loadMembers(serverId);
      const errorMessage = getErrorMessage(err, "Failed to ban member");
      if (isAuthError(errorMessage)) handleAuthError();
      return false;
    }
  }, [serverId, loadMembers]);

  const refresh = useCallback(() => {
    if (serverId) {
      loadMembers(serverId);
    }
  }, [serverId, loadMembers]);

  // Fonction helper pour obtenir le statut d'un utilisateur
  const getUserStatus = useCallback((userId: string): string => {
    return userStatuses.get(userId) || "offline";
  }, [userStatuses]);

  return {
    members,
    loading,
    error,
    refresh,
    getUserStatus,
    kickMember,
    banMember,
  };
}

