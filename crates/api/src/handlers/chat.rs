use crate::errors::ApiError;
use crate::state::AppState;
use application::chat::ChatEvent;
use axum::{
    extract::State,
    response::{sse::Event, Json, Sse},
};
use futures::Stream;
use std::convert::Infallible;

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<application::chat::ChatRequest>,
) -> Result<Json<application::chat::ChatResponse>, ApiError> {
    let usecase = application::chat::ChatWithApplicationUseCase::new(
        state.offre_repo.clone(),
        state.instance_repo.clone(),
        state.profil_repo.clone(),
        state.annexe_repo.clone(),
        state.chunk_repo.clone(),
        state.message_repo.clone(),
        state.embedder.clone(),
        state.llm_registry.as_ref().clone(),
    )
    .with_snapshot_repo(state.snapshot_repo.clone());

    let res = usecase
        .execute(req)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(res))
}

pub async fn chat_stream_handler(
    State(state): State<AppState>,
    Json(req): Json<application::chat::ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let usecase = application::chat::ChatWithApplicationUseCase::new(
        state.offre_repo.clone(),
        state.instance_repo.clone(),
        state.profil_repo.clone(),
        state.annexe_repo.clone(),
        state.chunk_repo.clone(),
        state.message_repo.clone(),
        state.embedder.clone(),
        state.llm_registry.as_ref().clone(),
    )
    .with_snapshot_repo(state.snapshot_repo.clone());

    let stream = usecase
        .execute_stream(req)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let sse_stream = futures::stream::unfold(stream, |mut s| async move {
        use futures::StreamExt;
        match s.next().await {
            Some(Ok(chat_event)) => {
                let event = match &chat_event {
                    ChatEvent::Status { content } => {
                        Event::default().event("status").data(content.clone())
                    }
                    ChatEvent::Token { content } => {
                        // Tokens are sent as plain data (default event type "message")
                        // for backward compatibility with existing SSE parsing
                        Event::default().data(content.clone())
                    }
                    ChatEvent::Mutation { .. } => {
                        let json =
                            serde_json::to_string(&chat_event).unwrap_or_else(|_| "{}".to_string());
                        Event::default().event("mutation").data(json)
                    }
                    ChatEvent::Done => Event::default().event("done").data("{}"),
                    ChatEvent::Error { message } => {
                        Event::default().event("error").data(message.clone())
                    }
                };
                Some((Ok(event), s))
            }
            Some(Err(e)) => {
                let event = Event::default().event("error").data(e.to_string());
                Some((Ok(event), s))
            }
            None => None,
        }
    });

    Ok(Sse::new(sse_stream))
}
