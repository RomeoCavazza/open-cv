"use client";

import { useState, useEffect, useCallback } from "react";
import {
  listServers,
  createServer as apiCreateServer,
  getServer as apiGetServer,
  updateServer as apiUpdateServer,
  deleteServer as apiDeleteServer,
  joinServer as apiJoinServer,
  leaveServer as apiLeaveServer,
  Server,
} from "@/lib/api-server";
import { handleAuthError, isAuthError, getErrorMessage } from "@/lib/auth/utils";

export function useServers() {
  const [servers, setServers] = useState<Server[]>([]);
  const [selectedServer, setSelectedServer] = useState<Server | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadServers = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await listServers();
      setServers(data);
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to load servers");
      if (isAuthError(errorMessage)) {
        setServers([]);
        setError(null);
        handleAuthError();
      } else {
        setError(errorMessage);
        setServers([]);
      }
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadServers();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const selectServer = useCallback((server: Server | null) => {
    setSelectedServer(server);
  }, []);

  const createServer = useCallback(async (name: string): Promise<Server | null> => {
    try {
      setError(null);
      const newServer = await apiCreateServer(name);
      await loadServers();
      setSelectedServer(newServer);
      return newServer;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to create server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, [loadServers]);

  const getServer = useCallback(async (id: string): Promise<Server | null> => {
    try {
      setError(null);
      const server = await apiGetServer(id);
      // Mettre à jour le serveur dans la liste si présent
      setServers((prev) =>
        prev.map((s) => (s.id === id ? server : s))
      );
      return server;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to get server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, []);

  const updateServer = useCallback(async (id: string, name: string): Promise<Server | null> => {
    let previousServers: Server[] = [];
    
    try {
      setError(null);
      
      // Optimisme UI : mise à jour immédiate
      setServers((prev) => {
        previousServers = [...prev];
        return prev.map((s) => (s.id === id ? { ...s, name, updated_at: new Date().toISOString() } : s));
      });
      
      // Si c'est le serveur sélectionné, le mettre à jour aussi
      setSelectedServer((prev) => {
        if (prev?.id === id) {
          return { ...prev, name, updated_at: new Date().toISOString() };
        }
        return prev;
      });

      const updatedServer = await apiUpdateServer(id, name);
      
      // Mise à jour avec les données du serveur
      setServers((prev) =>
        prev.map((s) => (s.id === id ? updatedServer : s))
      );
      
      setSelectedServer((prev) => {
        if (prev?.id === id) {
          return updatedServer;
        }
        return prev;
      });

      return updatedServer;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousServers.length > 0) {
        setServers(previousServers);
        const previousServer = previousServers.find((s) => s.id === id);
        if (previousServer && selectedServer?.id === id) {
          setSelectedServer(previousServer);
        }
      }
      
      const errorMessage = getErrorMessage(err, "Failed to update server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, [selectedServer]);

  const deleteServer = useCallback(async (id: string): Promise<boolean> => {
    let previousServers: Server[] = [];
    let previousSelected: Server | null = null;
    const wasSelected = selectedServer?.id === id;
    
    try {
      setError(null);
      
      // Optimisme UI : suppression immédiate
      setServers((prev) => {
        previousServers = [...prev];
        return prev.filter((s) => s.id !== id);
      });
      
      // Si c'était le serveur sélectionné, sélectionner le premier disponible
      if (wasSelected) {
        previousSelected = selectedServer;
        setServers((prev) => {
          const remaining = prev.filter((s) => s.id !== id);
          setSelectedServer(remaining.length > 0 ? remaining[0] : null);
          return prev;
        });
      }

      await apiDeleteServer(id);
      
      // Si c'était le serveur sélectionné, mettre à jour la sélection
      if (wasSelected) {
        setServers((prev) => {
          const remaining = prev.filter((s) => s.id !== id);
          setSelectedServer(remaining.length > 0 ? remaining[0] : null);
          return prev;
        });
      }
      
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousServers.length > 0) {
        setServers(previousServers);
        if (wasSelected && previousSelected) {
          setSelectedServer(previousSelected);
        }
      }
      
      const errorMessage = getErrorMessage(err, "Failed to delete server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return false;
      }
      setError(errorMessage);
      return false;
    }
  }, [selectedServer]);

  const joinServer = useCallback(async (id: string): Promise<Server | null> => {
    try {
      setError(null);
      await apiJoinServer(id);
      // Recharger la liste des serveurs pour inclure le nouveau
      await loadServers();
      // Sélectionner le serveur rejoint
      const joinedServer = await apiGetServer(id);
      if (joinedServer) {
        setSelectedServer(joinedServer);
      }
      return joinedServer;
    } catch (err) {
      const errorMessage = getErrorMessage(err, "Failed to join server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return null;
      }
      setError(errorMessage);
      return null;
    }
  }, [loadServers]);

  const leaveServer = useCallback(async (id: string): Promise<boolean> => {
    let previousServers: Server[] = [];
    let previousSelected: Server | null = null;
    const wasSelected = selectedServer?.id === id;
    
    try {
      setError(null);
      
      // Optimisme UI : suppression immédiate
      setServers((prev) => {
        previousServers = [...prev];
        return prev.filter((s) => s.id !== id);
      });
      
      // Si c'était le serveur sélectionné, sélectionner le premier disponible
      if (wasSelected) {
        previousSelected = selectedServer;
        setServers((prev) => {
          const remaining = prev.filter((s) => s.id !== id);
          setSelectedServer(remaining.length > 0 ? remaining[0] : null);
          return prev;
        });
      }

      await apiLeaveServer(id);
      return true;
    } catch (err) {
      // Rollback en cas d'erreur
      if (previousServers.length > 0) {
        setServers(previousServers);
        if (wasSelected && previousSelected) {
          setSelectedServer(previousSelected);
        }
      }
      
      const errorMessage = getErrorMessage(err, "Failed to leave server");
      if (isAuthError(errorMessage)) {
        handleAuthError();
        return false;
      }
      setError(errorMessage);
      return false;
    }
  }, [selectedServer]);

  const refresh = useCallback(() => {
    loadServers();
  }, [loadServers]);

  return {
    servers,
    selectedServer,
    loading,
    error,
    selectServer,
    createServer,
    getServer,
    updateServer,
    deleteServer,
    joinServer,
    leaveServer,
    refresh,
  };
}

