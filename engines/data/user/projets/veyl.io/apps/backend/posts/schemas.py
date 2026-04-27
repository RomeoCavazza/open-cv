# posts/schemas.py
from pydantic import BaseModel, Field
from typing import Optional, Dict, Any, List
from datetime import datetime

class PostBase(BaseModel):
    platform_id: int
    author: Optional[str] = None
    caption: Optional[str] = None
    hashtags: Optional[List[str]] = None
    metrics: Optional[Dict[str, Any]] = None
    posted_at: Optional[datetime] = None
    language: Optional[str] = None
    media_url: Optional[str] = None
    sentiment: Optional[float] = None
    score: Optional[float] = Field(default=0.0)
    score_trend: Optional[float] = Field(default=0.0)

class PostCreate(PostBase):
    id: str

class PostUpdate(BaseModel):
    author: Optional[str] = None
    caption: Optional[str] = None
    hashtags: Optional[List[str]] = None
    metrics: Optional[Dict[str, Any]] = None
    posted_at: Optional[datetime] = None
    language: Optional[str] = None
    media_url: Optional[str] = None
    sentiment: Optional[float] = None
    score: Optional[float] = None
    score_trend: Optional[float] = None

class PostResponse(PostBase):
    id: str
    platform_id: int
    fetched_at: Optional[datetime] = None
    
    class Config:
        from_attributes = True
